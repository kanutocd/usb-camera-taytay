//! Small Linux V4L2 streaming boundary using driver-owned mmap buffers.
//!
//! A dequeued frame is copied before its driver buffer is queued again. The
//! mapping and file descriptor stay owned by `Capture` and are released in
//! `Drop`, including when setup or frame validation fails.

use std::{
    ffi::c_void,
    fs::OpenOptions,
    io,
    os::{fd::AsRawFd, unix::fs::OpenOptionsExt},
    path::Path,
    ptr,
};

const VIDEO_CAPTURE: u32 = 1;
const MEMORY_MMAP: u32 = 1;
const BUFFER_COUNT: u32 = 4;
const VIDIOC_S_FMT: libc::c_ulong = 0xc0d0_5605;
const VIDIOC_REQBUFS: libc::c_ulong = 0xc014_5608;
const VIDIOC_QUERYBUF: libc::c_ulong = 0xc058_5609;
const VIDIOC_QBUF: libc::c_ulong = 0xc058_560f;
const VIDIOC_DQBUF: libc::c_ulong = 0xc058_5611;
const VIDIOC_STREAMON: libc::c_ulong = 0x4004_5612;
const VIDIOC_STREAMOFF: libc::c_ulong = 0x4004_5613;

#[repr(C)]
struct RequestBuffers {
    count: u32,
    type_: u32,
    memory: u32,
    capabilities: u32,
    flags: u8,
    reserved: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PixelFormat {
    width: u32,
    height: u32,
    pixel_format: u32,
    field: u32,
    bytes_per_line: u32,
    size_image: u32,
    colorspace: u32,
    private: u32,
    flags: u32,
    ycbcr_enc: u32,
    quantization: u32,
    xfer_func: u32,
}

#[repr(C)]
union FormatData {
    pixel: PixelFormat,
    padding: [u64; 25],
}

#[repr(C)]
struct Format {
    type_: u32,
    data: FormatData,
}

#[repr(C)]
struct TimeVal {
    sec: libc::c_long,
    usec: libc::c_long,
}

#[repr(C)]
struct Buffer {
    index: u32,
    type_: u32,
    bytes_used: u32,
    flags: u32,
    field: u32,
    timestamp: TimeVal,
    timecode: [u32; 4],
    sequence: u32,
    memory: u32,
    offset: u32,
    length: u32,
    reserved2: u32,
    request_fd: i32,
}

struct Mapping {
    address: *mut libc::c_void,
    length: usize,
}

impl Drop for Mapping {
    fn drop(&mut self) {
        // SAFETY: `address` and `length` are the exact successful mmap result.
        unsafe { libc::munmap(self.address, self.length) };
    }
}

/// Streaming capture configured to one fixed pixel format and frame size.
pub struct Capture {
    device: std::fs::File,
    mappings: Vec<Mapping>,
    streaming: bool,
    kernel: Box<dyn Kernel>,
}

trait Kernel {
    fn ioctl(&self, fd: libc::c_int, request: libc::c_ulong, value: *mut c_void) -> io::Result<()>;
    unsafe fn map(
        &self,
        fd: libc::c_int,
        length: usize,
        offset: libc::off_t,
    ) -> io::Result<*mut c_void>;
}

struct LinuxKernel;

impl Kernel for LinuxKernel {
    fn ioctl(&self, fd: libc::c_int, request: libc::c_ulong, value: *mut c_void) -> io::Result<()> {
        // SAFETY: the caller supplies the kernel-defined structure for this request.
        if unsafe { libc::ioctl(fd, request, value) } < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    unsafe fn map(
        &self,
        fd: libc::c_int,
        length: usize,
        offset: libc::off_t,
    ) -> io::Result<*mut c_void> {
        // SAFETY: V4L2 QUERYBUF supplies the mapping length and offset.
        let address = unsafe {
            libc::mmap(
                ptr::null_mut(),
                length,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                offset,
            )
        };
        if address == libc::MAP_FAILED {
            Err(io::Error::last_os_error())
        } else {
            Ok(address)
        }
    }
}

impl Capture {
    /// Configure the requested mode and start streaming with four mmap buffers.
    pub fn open(
        device_path: &Path,
        width: u32,
        height: u32,
        pixel_format: u32,
    ) -> io::Result<Self> {
        if width == 0 || height == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid capture size",
            ));
        }
        let device = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_NONBLOCK)
            .open(device_path)?;
        Self::open_device(device, width, height, pixel_format, Box::new(LinuxKernel))
    }

    fn open_device(
        device: std::fs::File,
        width: u32,
        height: u32,
        pixel_format: u32,
        kernel: Box<dyn Kernel>,
    ) -> io::Result<Self> {
        if width == 0 || height == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid capture size",
            ));
        }
        let fd = device.as_raw_fd();
        let mut format = Format {
            type_: VIDEO_CAPTURE,
            data: FormatData {
                pixel: PixelFormat {
                    width,
                    height,
                    pixel_format,
                    field: 1,
                    bytes_per_line: 0,
                    size_image: 0,
                    colorspace: 0,
                    private: 0,
                    flags: 0,
                    ycbcr_enc: 0,
                    quantization: 0,
                    xfer_func: 0,
                },
            },
        };
        kernel.ioctl(fd, VIDIOC_S_FMT, (&mut format as *mut Format).cast())?;
        // SAFETY: S_FMT initialized the pixel format union member.
        let negotiated = unsafe { format.data.pixel };
        if negotiated.width != width
            || negotiated.height != height
            || negotiated.pixel_format != pixel_format
        {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "device did not accept requested capture mode",
            ));
        }

        let mut request = RequestBuffers {
            count: BUFFER_COUNT,
            type_: VIDEO_CAPTURE,
            memory: MEMORY_MMAP,
            capabilities: 0,
            flags: 0,
            reserved: [0; 3],
        };
        kernel.ioctl(
            fd,
            VIDIOC_REQBUFS,
            (&mut request as *mut RequestBuffers).cast(),
        )?;
        if request.count < 2 {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "device returned too few capture buffers",
            ));
        }

        let mut capture = Self {
            device,
            mappings: Vec::with_capacity(request.count as usize),
            streaming: false,
            kernel,
        };
        for index in 0..request.count {
            let mut buffer = empty_buffer(index);
            capture
                .kernel
                .ioctl(fd, VIDIOC_QUERYBUF, (&mut buffer as *mut Buffer).cast())?;
            let length = usize::try_from(buffer.length).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "invalid V4L2 buffer length")
            })?;
            if length == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "empty V4L2 buffer",
                ));
            }
            // SAFETY: QUERYBUF supplied the driver-owned offset and length for mmap.
            let address = unsafe {
                capture
                    .kernel
                    .map(fd, length, buffer.offset as libc::off_t)?
            };
            capture.mappings.push(Mapping { address, length });
            capture
                .kernel
                .ioctl(fd, VIDIOC_QBUF, (&mut buffer as *mut Buffer).cast())?;
        }
        let mut kind = VIDEO_CAPTURE;
        capture
            .kernel
            .ioctl(fd, VIDIOC_STREAMON, (&mut kind as *mut u32).cast())?;
        capture.streaming = true;
        Ok(capture)
    }

    /// Wait for one complete driver frame, copy it, and return its buffer to V4L2.
    pub fn next_frame(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = empty_buffer(0);
        self.kernel.ioctl(
            self.device.as_raw_fd(),
            VIDIOC_DQBUF,
            (&mut buffer as *mut Buffer).cast(),
        )?;
        let result = frame_bytes(
            &self.mappings,
            buffer.index as usize,
            buffer.bytes_used as usize,
        );
        let queued = self.kernel.ioctl(
            self.device.as_raw_fd(),
            VIDIOC_QBUF,
            (&mut buffer as *mut Buffer).cast(),
        );
        match (result, queued) {
            (Ok(frame), Ok(())) => Ok(frame),
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
        }
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        if self.streaming {
            let mut kind = VIDEO_CAPTURE;
            // SAFETY: fd is open and streaming was successfully started.
            let _ = self.kernel.ioctl(
                self.device.as_raw_fd(),
                VIDIOC_STREAMOFF,
                (&mut kind as *mut u32).cast(),
            );
        }
    }
}

fn empty_buffer(index: u32) -> Buffer {
    Buffer {
        index,
        type_: VIDEO_CAPTURE,
        bytes_used: 0,
        flags: 0,
        field: 0,
        timestamp: TimeVal { sec: 0, usec: 0 },
        timecode: [0; 4],
        sequence: 0,
        memory: MEMORY_MMAP,
        offset: 0,
        length: 0,
        reserved2: 0,
        request_fd: 0,
    }
}

fn frame_bytes(mappings: &[Mapping], index: usize, bytes_used: usize) -> io::Result<Vec<u8>> {
    let mapping = mappings.get(index).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "driver returned invalid buffer index",
        )
    })?;
    if bytes_used == 0 || bytes_used > mapping.length {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "driver returned invalid frame length",
        ));
    }
    // SAFETY: the buffer is dequeued, and bytes_used is bounded by its mapping.
    let frame = unsafe { std::slice::from_raw_parts(mapping.address.cast::<u8>(), bytes_used) };
    Ok(frame.to_vec())
}

#[cfg(test)]
mod tests {
    use super::{
        BUFFER_COUNT, Buffer, Capture, Format, Kernel, Mapping, RequestBuffers, VIDIOC_DQBUF,
        VIDIOC_QBUF, VIDIOC_QUERYBUF, VIDIOC_REQBUFS, VIDIOC_S_FMT, VIDIOC_STREAMOFF,
        VIDIOC_STREAMON, frame_bytes,
    };
    use std::{cell::RefCell, ffi::c_void, fs::OpenOptions, io, os::fd::AsRawFd, ptr};

    struct MockKernel {
        fail_request: Option<libc::c_ulong>,
        fail_qbuf_call: Option<usize>,
        qbuf_calls: RefCell<usize>,
        buffer_count: u32,
        buffer_length: u32,
        frame_index: u32,
        frame_length: u32,
        mismatch_mode: bool,
        fail_map: bool,
    }

    impl MockKernel {
        fn new() -> Self {
            Self {
                fail_request: None,
                fail_qbuf_call: None,
                qbuf_calls: RefCell::new(0),
                buffer_count: BUFFER_COUNT,
                buffer_length: 4,
                frame_index: 0,
                frame_length: 3,
                mismatch_mode: false,
                fail_map: false,
            }
        }
    }

    impl Kernel for MockKernel {
        fn ioctl(
            &self,
            _fd: libc::c_int,
            request: libc::c_ulong,
            value: *mut c_void,
        ) -> io::Result<()> {
            if self.fail_request == Some(request) {
                return Err(io::Error::from_raw_os_error(libc::EIO));
            }
            // SAFETY: `Capture` passes the V4L2 structure associated with each request.
            unsafe {
                match request {
                    VIDIOC_S_FMT => {
                        let format = &mut *value.cast::<Format>();
                        if self.mismatch_mode {
                            format.data.pixel.width += 1;
                        }
                    }
                    VIDIOC_REQBUFS => (*value.cast::<RequestBuffers>()).count = self.buffer_count,
                    VIDIOC_QUERYBUF => {
                        let buffer = &mut *value.cast::<Buffer>();
                        buffer.length = self.buffer_length;
                        buffer.offset = buffer.index * self.buffer_length;
                    }
                    VIDIOC_QBUF => {
                        let mut calls = self.qbuf_calls.borrow_mut();
                        *calls += 1;
                        if self.fail_qbuf_call == Some(*calls) {
                            return Err(io::Error::from_raw_os_error(libc::EIO));
                        }
                    }
                    VIDIOC_DQBUF => {
                        let buffer = &mut *value.cast::<Buffer>();
                        buffer.index = self.frame_index;
                        buffer.bytes_used = self.frame_length;
                    }
                    VIDIOC_STREAMON | VIDIOC_STREAMOFF => {}
                    _ => panic!("unexpected V4L2 ioctl request: {request:#x}"),
                }
            }
            Ok(())
        }

        unsafe fn map(
            &self,
            _fd: libc::c_int,
            length: usize,
            _offset: libc::off_t,
        ) -> io::Result<*mut c_void> {
            if self.fail_map {
                return Err(io::Error::from_raw_os_error(libc::ENOMEM));
            }
            // Anonymous memory models the writable mapping returned by a V4L2 driver.
            let address = unsafe {
                libc::mmap(
                    ptr::null_mut(),
                    length,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                    -1,
                    0,
                )
            };
            if address == libc::MAP_FAILED {
                return Err(io::Error::last_os_error());
            }
            // SAFETY: the anonymous mapping is valid for `length` bytes.
            unsafe {
                ptr::copy_nonoverlapping([1_u8, 2, 3, 4].as_ptr(), address.cast(), length.min(4));
            }
            Ok(address)
        }
    }

    fn device() -> std::fs::File {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/null")
            .unwrap()
    }

    fn capture(kernel: MockKernel) -> io::Result<Capture> {
        Capture::open_device(
            device(),
            640,
            480,
            u32::from_le_bytes(*b"MJPG"),
            Box::new(kernel),
        )
    }

    #[test]
    fn configures_buffers_copies_frame_and_stops_streaming() {
        let mut capture = capture(MockKernel::new()).unwrap();
        assert_eq!(capture.mappings.len(), BUFFER_COUNT as usize);
        assert_eq!(capture.next_frame().unwrap(), [1, 2, 3]);
        drop(capture);
    }

    #[test]
    fn rejects_invalid_dimensions_negotiation_and_buffer_counts() {
        let error = Capture::open_device(device(), 0, 480, 0, Box::new(MockKernel::new()))
            .err()
            .unwrap();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);

        let mut kernel = MockKernel::new();
        kernel.mismatch_mode = true;
        assert_eq!(
            capture(kernel).err().unwrap().kind(),
            io::ErrorKind::Unsupported
        );

        let mut kernel = MockKernel::new();
        kernel.buffer_count = 1;
        assert_eq!(
            capture(kernel).err().unwrap().kind(),
            io::ErrorKind::Unsupported
        );

        let mut kernel = MockKernel::new();
        kernel.buffer_length = 0;
        assert_eq!(
            capture(kernel).err().unwrap().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn propagates_setup_and_frame_ioctl_failures() {
        for request in [
            VIDIOC_S_FMT,
            VIDIOC_REQBUFS,
            VIDIOC_QUERYBUF,
            VIDIOC_QBUF,
            VIDIOC_STREAMON,
        ] {
            let mut kernel = MockKernel::new();
            kernel.fail_request = Some(request);
            assert!(capture(kernel).is_err(), "request {request:#x}");
        }

        let mut capture = capture(MockKernel {
            fail_request: Some(VIDIOC_DQBUF),
            ..MockKernel::new()
        })
        .unwrap();
        assert_eq!(
            capture.next_frame().unwrap_err().raw_os_error(),
            Some(libc::EIO)
        );
    }

    #[test]
    fn requeues_invalid_frames_and_reports_queue_failures() {
        let mut kernel = MockKernel::new();
        kernel.frame_index = BUFFER_COUNT;
        let mut first_capture = capture(kernel).unwrap();
        assert_eq!(
            first_capture.next_frame().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );

        let mut kernel = MockKernel::new();
        kernel.frame_length = 0;
        let mut second_capture = capture(kernel).unwrap();
        assert_eq!(
            second_capture.next_frame().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );

        let mut kernel = MockKernel::new();
        kernel.fail_qbuf_call = Some(BUFFER_COUNT as usize + 1);
        let mut third_capture = capture(kernel).unwrap();
        assert_eq!(
            third_capture.next_frame().unwrap_err().raw_os_error(),
            Some(libc::EIO)
        );
    }

    #[test]
    fn reports_mapping_errors_and_exercises_linux_mapping_boundary() {
        let mut kernel = MockKernel::new();
        kernel.fail_map = true;
        assert_eq!(
            capture(kernel).err().unwrap().raw_os_error(),
            Some(libc::ENOMEM)
        );

        let backing_path =
            std::env::temp_dir().join(format!("usb-camera-taytay-v4l2-map-{}", std::process::id()));
        let backing = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&backing_path)
            .unwrap();
        backing.set_len(4).unwrap();
        let linux = super::LinuxKernel;
        // SAFETY: the writable backing file supports a shared mapping, unmapped below.
        let address = unsafe { linux.map(backing.as_raw_fd(), 4, 0).unwrap() };
        // SAFETY: the mapping was created above with length four.
        unsafe { libc::munmap(address, 4) };
        assert!(linux.ioctl(-1, 0, ptr::null_mut()).is_err());
        // SAFETY: an invalid descriptor must make mmap fail before yielding an address.
        assert!(unsafe { linux.map(-1, 4, 0) }.is_err());
        std::fs::remove_file(backing_path).unwrap();
    }

    #[test]
    fn copies_only_the_reported_bytes_from_a_valid_mapping() {
        let data = [1_u8, 2, 3, 4];
        let mappings = [Mapping {
            address: data.as_ptr().cast_mut().cast(),
            length: data.len(),
        }];
        assert_eq!(frame_bytes(&mappings, 0, 3).unwrap(), [1, 2, 3]);
        std::mem::forget(mappings);
    }

    #[test]
    fn rejects_invalid_frame_indices_and_lengths() {
        let mapping = Mapping {
            address: ptr::null_mut(),
            length: 4,
        };
        assert_eq!(
            frame_bytes(&[mapping], 1, 1).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        let mapping = Mapping {
            address: ptr::null_mut(),
            length: 4,
        };
        assert_eq!(
            frame_bytes(&[mapping], 0, 5).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        let mapping = Mapping {
            address: ptr::null_mut(),
            length: 4,
        };
        assert_eq!(
            frame_bytes(&[mapping], 0, 0).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
}
