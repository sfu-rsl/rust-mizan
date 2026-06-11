// Right-hand "On this page" outline for the current chapter.
// Self-contained: no preprocessor or external dependency.
(function () {
  function build() {
    var main = document.querySelector("main");
    if (!main) return;

    var headers = main.querySelectorAll("h2, h3");
    var withIds = Array.prototype.filter.call(headers, function (h) {
      return h.id;
    });
    if (withIds.length < 2) return; // not worth a TOC on short pages

    var nav = document.createElement("nav");
    nav.className = "pagetoc";
    var title = document.createElement("div");
    title.className = "pagetoc-title";
    title.textContent = "On this page";
    nav.appendChild(title);

    withIds.forEach(function (h) {
      var a = document.createElement("a");
      a.href = "#" + h.id;
      a.textContent = h.textContent;
      a.className = "pagetoc-" + h.tagName.toLowerCase();
      a.dataset.id = h.id;
      nav.appendChild(a);
    });

    var page = document.querySelector(".page") || main.parentNode;
    page.appendChild(nav);

    var links = nav.querySelectorAll("a");
    function spy() {
      var pos = window.scrollY + 120;
      var current = withIds[0].id;
      withIds.forEach(function (h) {
        if (h.offsetTop <= pos) current = h.id;
      });
      links.forEach(function (l) {
        l.classList.toggle("active", l.dataset.id === current);
      });
    }
    window.addEventListener("scroll", spy, { passive: true });
    window.addEventListener("resize", spy, { passive: true });
    spy();
  }

  if (document.readyState !== "loading") build();
  else document.addEventListener("DOMContentLoaded", build);
})();
