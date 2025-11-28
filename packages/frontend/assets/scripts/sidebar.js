export class Sidebar {
  // containerEl: the sidebar container element (where the UI will be rendered).
  // options:
  //   viewport: an existing Viewport instance (optional)
  //   viewportContainer: element to create a viewport in (optional; used if viewport not provided)
  //   createViewport: function(container) -> Viewport (optional; defaults to new Viewport(container))
  //   nodeDefs: array of node definitions
  constructor(containerEl, options = {}) {
    if (!(containerEl instanceof HTMLElement)) {
      throw new Error("Sidebar requires a DOM element as first argument");
    }
    this.containerEl = containerEl;
    this.nodeDefs = options.nodeDefs || [];
    this._bound = new Map();

    // Setup or use viewport
    this.vp = null;
    if (options.viewport instanceof Viewport) {
      this.vp = options.viewport;
    } else if (options.viewportContainer instanceof HTMLElement) {
      const factory =
        typeof options.createViewport === "function"
          ? options.createViewport
          : (c) => new Viewport(c, options.viewportOptions || {});
      this.vp = factory(options.viewportContainer);
    } else {
      // no viewport provided, create one in a sibling element if available
      // try to find a child with class 'viewport-container' inside the same app parent
      const defaultViewportContainer = document.querySelector(
        ".viewport-container",
      );
      if (defaultViewportContainer) {
        const factory =
          typeof options.createViewport === "function"
            ? options.createViewport
            : (c) => new Viewport(c, options.viewportOptions || {});
        this.vp = factory(defaultViewportContainer);
      }
    }

    // Build sidebar UI
    this._render();

    // initial list
    this.renderList();

    // attach drag/drop handlers to viewport container if viewport exists
    if (this.vp && this.vp.container instanceof HTMLElement) {
      this._addDropHandlers(this.vp.container);
    }

    // expose for debugging
    this.rootSidebarElement = this.rootEl;
  }

  setNodeDefs(nodeDefs) {
    this.nodeDefs = Array.isArray(nodeDefs) ? nodeDefs.slice() : [];
    this.renderList();
  }

  addNodeDef(def) {
    this.nodeDefs.push(def);
    this.renderList();
  }

  // create the sidebar DOM structure
  _render() {
    // Clear container
    this.containerEl.innerHTML = "";

    // root
    const root = document.createElement("div");
    root.className = "sidebar-root";
    root.style.display = "flex";
    root.style.flexDirection = "column";
    root.style.gap = "8px";
    this.containerEl.appendChild(root);
    this.rootEl = root;

    // header with search
    const header = document.createElement("div");
    header.className = "sidebar-header";
    header.style.display = "flex";
    header.style.alignItems = "center";
    header.style.gap = "8px";
    root.appendChild(header);

    const search = document.createElement("input");
    search.type = "search";
    search.placeholder = "Search nodes...";
    search.className = "sidebar-search";
    search.style.width = "100%";
    search.style.padding = "8px";
    search.style.borderRadius = "6px";
    search.style.border = "none";
    search.style.background = "rgba(0,0,0,0.15)";
    search.style.color = "#fff";
    header.appendChild(search);
    this.searchInput = search;

    // list
    const list = document.createElement("div");
    list.className = "node-list";
    list.style.overflow = "auto";
    list.style.padding = "4px 2px 12px 2px";
    list.style.flex = "1 1 auto";
    root.appendChild(list);
    this.listEl = list;

    // listeners
    const onSearch = (e) => this.renderList(e.target.value || "");
    this.searchInput.addEventListener("input", onSearch);
    this._bound.set("search-input", onSearch);
  }

  // create a DOM node item for a definition
  _makeNodeItem(def) {
    const el = document.createElement("div");
    el.className = "node-item";
    el.style.display = "block";
    el.style.cursor = "grab";
    el.style.userSelect = "none";
    el.style.padding = "10px";
    el.style.borderRadius = "8px";
    el.style.marginBottom = "8px";
    el.style.background = "linear-gradient(180deg,#182034,#1f2940)";
    el.setAttribute("draggable", "true");
    el.dataset.def = JSON.stringify(def);

    const h = document.createElement("h4");
    h.textContent = def.label || def.id || "Node";
    h.style.margin = "0 0 6px 0";
    h.style.fontSize = "15px";
    h.style.color = "#fff";
    el.appendChild(h);

    if (def.description) {
      const p = document.createElement("p");
      p.textContent = def.description;
      p.style.margin = "0";
      p.style.fontSize = "12px";
      p.style.color = "#9aa3b2";
      el.appendChild(p);
    }

    if (Array.isArray(def.params) && def.params.length) {
      const meta = document.createElement("div");
      meta.style.marginTop = "8px";
      meta.style.display = "flex";
      meta.style.gap = "6px";
      meta.style.flexWrap = "wrap";
      def.params.forEach((pp) => {
        const pill = document.createElement("div");
        pill.textContent = (pp.type === "input" ? "in: " : "out: ") + pp.name;
        pill.style.fontSize = "11px";
        pill.style.padding = "4px 8px";
        pill.style.borderRadius = "999px";
        pill.style.background = "rgba(255,255,255,0.03)";
        pill.style.color = "#cfe5ff";
        meta.appendChild(pill);
      });
      el.appendChild(meta);
    }

    // drag handlers
    el.addEventListener("dragstart", (ev) => {
      try {
        ev.dataTransfer.setData("application/json", JSON.stringify(def));
      } catch (err) {
        // ignore
      }
      // optional drag image
      if (ev.dataTransfer.setDragImage) {
        const img = document.createElement("canvas");
        img.width = 160;
        img.height = 40;
        const ctx = img.getContext("2d");
        ctx.fillStyle = "#253042";
        ctx.fillRect(0, 0, img.width, img.height);
        ctx.fillStyle = "#fff";
        ctx.font = "14px sans-serif";
        ctx.fillText(def.label || def.id || "Node", 10, 26);
        ev.dataTransfer.setDragImage(img, 10, 10);
      }
    });

    return el;
  }

  // render node list filtered by the search query
  renderList(filter = "") {
    this.listEl.innerHTML = "";
    const q = String(filter || "")
      .trim()
      .toLowerCase();
    for (const def of this.nodeDefs) {
      if (
        !q ||
        (def.label && def.label.toLowerCase().includes(q)) ||
        (def.description && def.description.toLowerCase().includes(q))
      ) {
        this.listEl.appendChild(this._makeNodeItem(def));
      }
    }
  }

  // attach drop handlers to the given element (viewport container)
  _addDropHandlers(dropTarget) {
    if (!dropTarget || !(dropTarget instanceof HTMLElement)) return;
    const onDragOver = (ev) => {
      ev.preventDefault();
    };
    const onDrop = (ev) => {
      ev.preventDefault();
      if (!this.vp) return;
      try {
        const raw = ev.dataTransfer.getData("application/json");
        if (!raw) return;
        const def = JSON.parse(raw);

        // compute local canvas coordinates of drop
        const rect = this.vp.canvas.getBoundingClientRect();
        const cx = ev.clientX - rect.left;
        const cy = ev.clientY - rect.top;

        const pos = this.vp.toEditor(cx, cy);

        // create node using viewport.createNode if available, otherwise directly call createNode
        if (typeof this.vp.createNode === "function") {
          this.vp.createNode(
            pos.x,
            pos.y,
            def.label || def.id,
            def.params || [],
          );
        } else {
          // fallback: instantiate Node & add
          const NodeClass = this.vp.Node || null;
          if (typeof this.vp.addNode === "function") {
            // attempt to create a node-like object
            this.vp.addNode({
              x: pos.x,
              y: pos.y,
              label: def.label || def.id,
              params: def.params || [],
            });
          }
        }

        // emit a DOM CustomEvent for consumers
        const evDetail = { def, pos };
        const dropped = new CustomEvent("sidebar-node-drop", {
          detail: evDetail,
        });
        dropTarget.dispatchEvent(dropped);
      } catch (err) {
        // ignore malformed drops
        console.error("Sidebar drop error", err);
      }
    };

    dropTarget.addEventListener("dragover", onDragOver);
    dropTarget.addEventListener("drop", onDrop);

    this._bound.set("drop-target", { el: dropTarget, onDragOver, onDrop });
  }

  // If you created the viewport later or want to rebind drop handlers:
  attachToViewport(viewportInstance) {
    if (!(viewportInstance instanceof Viewport)) return;
    // remove previous drop handlers if any
    const prev = this._bound.get("drop-target");
    if (prev && prev.el) {
      prev.el.removeEventListener("dragover", prev.onDragOver);
      prev.el.removeEventListener("drop", prev.onDrop);
    }
    this.vp = viewportInstance;
    this._addDropHandlers(this.vp.container);
  }

  // remove listeners and DOM created by Sidebar
  destroy() {
    // remove search listener
    const onSearch = this._bound.get("search-input");
    if (onSearch) this.searchInput.removeEventListener("input", onSearch);

    // remove drop listener
    const prev = this._bound.get("drop-target");
    if (prev && prev.el) {
      prev.el.removeEventListener("dragover", prev.onDragOver);
      prev.el.removeEventListener("drop", prev.onDrop);
    }

    // clear DOM
    if (this.rootEl && this.rootEl.parentElement) {
      this.rootEl.parentElement.removeChild(this.rootEl);
    }

    // do not destroy viewport here (consumer may still use it)
    this._bound.clear();
  }
}
