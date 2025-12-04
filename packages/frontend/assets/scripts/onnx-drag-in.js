// A script that is applied to the OnnxDragIn with a serach bar input and a container of nodes.
// It offers search capabilities and draggable Nodes with can be added to the OnnxViewport.
// This only works with when the window.activeOnnxViewport is set.
// This approach offers better readablily as opposed to all java script.
// Also this could be replaced eventually with a native rendering solution from dixous
class OnnxDragIn {
  constructor(rootelm, search, nodeContainer) {
    if (!(rootelm instanceof HTMLElement)) {
      throw new Error(
        "Yout first argument is no valid DOM element. Please provide the drag in element itself.",
      );
    }
    if (!(search instanceof HTMLElement)) {
      throw new Error(
        "Your second argument is no valid DOM element. Please provide a search type input.",
      );
    }
    if (!(nodeContainer instanceof HTMLElement)) {
      throw new Error(
        "Your third argument is no valid DOM element. Please provide the container of the nodes.",
      );
    }
    this.rootelm = rootelm;
    this.search = search;
    this.draggingChild = null;
    this.nodeContainer = nodeContainer;
    // this.nodeContainer.style.display = "relative";
    this.nodes = Array.from(this.nodeContainer.children);
    this._ondrag = this._ondrag.bind(this);
    this._ondrop = this._ondrop.bind(this);
    this.start_drag = null;
    this._attachListeners();
  }
  _ondrop(e) {
    this.draggingChild.style.left = "initial";
    this.draggingChild.style.top = "initial";
    this.draggingChild.style.position = "initial";
    window.activeOnnxViewport.addNodeFromId(this.draggingChild.id, {
      position: { x: e.x, y: e.y },
    });
    this.draggingChild = null;
  }
  _ondrag(e) {
    let rect = this.draggingChild.getBoundingClientRect();
    this.draggingChild.style.left = `${e.x - rect.width / 2}px`;
    this.draggingChild.style.top = `${e.y - rect.height / 2}px`;
  }
  _attachListeners() {
    this.nodes.forEach((child) => {
      console.log("hello");
      child.addEventListener("mousedown", (e) => {
        this.draggingChild = child;
        // let container_rect = this.nodeContainer.getBoundingClientRect();
        // let node_rect = this.draggingChild.getBoundingClientRect();
        // this.start_drag = { x: e.x, y: e.y };
        this.draggingChild.style.position = "fixed";
        window.addEventListener("mousemove", this._ondrag);
      });
    });
    window.addEventListener("mouseup", (e) => {
      if (this.draggingChild) {
        window.removeEventListener("mousemove", this._ondrag);
        this._ondrop(e);
      }
    });
  }
}

window.OnnxDragIn = OnnxDragIn;
