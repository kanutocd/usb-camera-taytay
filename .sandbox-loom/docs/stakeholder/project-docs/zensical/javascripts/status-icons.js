(function () {
  "use strict";

  var STATUS_ICONS = {
    completed: {
      className: "lucide-circle-check",
      markup: '<circle cx="12" cy="12" r="10"></circle><path d="m16 9-5.5 5.5L8 12"></path>'
    },
    pending: {
      className: "lucide-circle-dashed",
      markup: '<path d="M10.1 2.182a10 10 0 0 1 3.8 0"></path><path d="M13.9 21.818a10 10 0 0 1-3.8 0"></path><path d="M17.609 3.721a10 10 0 0 1 2.69 2.7"></path><path d="M2.182 13.9a10 10 0 0 1 0-3.8"></path><path d="M20.279 17.609a10 10 0 0 1-2.7 2.69"></path><path d="M21.818 10.1a10 10 0 0 1 0 3.8"></path><path d="M3.721 6.391a10 10 0 0 1 2.7-2.69"></path><path d="M6.391 20.279a10 10 0 0 1-2.69-2.7"></path>'
    },
    in_progress: {
      className: "lucide-rotate-cw-fading-clock",
      markup: '<path d="M12 3a9.75 9.75 0 0 1 6.74 2.74"></path><path d="M18.74 5.74 21 8"></path><path d="M21 8V3"></path><path d="M7.5 19.794c-6-3.464-6-12.124 0-15.588"></path><path d="M7.5 4.206A9 9 0 0 1 12 3"></path><path d="M12 7v5l4 2"></path><path d="M14 20.775A9 9 0 0 1 12 21"></path><path d="M19 17.656a9 9 0 0 1-1.5 1.456"></path><path d="M21 12a9 9 0 0 1-.228 2"></path><path d="M21 8h-5"></path>'
    }
  };
  var READY = "data-status-icon-ready";

  function createIcon(definition) {
    var icon = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    icon.setAttribute("xmlns", "http://www.w3.org/2000/svg");
    icon.setAttribute("fill", "none");
    icon.setAttribute("stroke", "currentColor");
    icon.setAttribute("stroke-linecap", "round");
    icon.setAttribute("stroke-linejoin", "round");
    icon.setAttribute("stroke-width", "2");
    icon.setAttribute("viewBox", "0 0 24 24");
    icon.setAttribute("class", "lucide " + definition.className);
    icon.setAttribute("aria-hidden", "true");
    icon.innerHTML = definition.markup;
    return icon;
  }

  function enhanceTable(table) {
    table.querySelectorAll("tbody td").forEach(function (cell) {
      var status = cell.textContent.trim();
      var definition = STATUS_ICONS[status];

      if (!definition || cell.hasAttribute(READY)) {
        return;
      }

      cell.setAttribute("data-status", status);
      cell.setAttribute("aria-label", status);
      cell.appendChild(createIcon(definition));
      cell.setAttribute(READY, "");
    });
  }

  function enhance() {
    document.querySelectorAll("article table").forEach(function (table) {
      enhanceTable(table);
    });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhance, { once: true });
  } else {
    enhance();
  }

  new MutationObserver(enhance).observe(document.body, {
    childList: true,
    subtree: true
  });
}());
