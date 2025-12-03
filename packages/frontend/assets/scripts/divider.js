class Divider {
  constructor(divider, orientation) {
    if (!(divider instanceof HTMLElement)) {
      throw new Error("Divider constructor requires a DOM container element");
    }
    this.divider = divider;
    this.orientation = orientation;
    this.firstPane = this.divider.previousSibling;
    this.secondPane = this.divider.nextSibling;

    console.log(this.firstPane, this.secondPane);

    this._startFirstPane = null;
    this._startCursor = null;

    this._onMouseDown = this._onMouseDown.bind(this);
    this._onMouseMove = this._onMouseMove.bind(this);
    this._onMouseMove = this._onMouseUp.bind(this);

    switch (orientation) {
      case "h":
        this._getStartCursor = this._getHorizontalStartCursor;
        this._getStartFirstPane = this._getHorizontalStartFirstPane;
        this._getCursorStyle = this._getHorizontalCursorStyle;
        this.divider.classList.add("horizontal");
        this._setSize = this._setHorizontalSize;
        break;

      case "v":
        this._getStartCursor = this._getVerticalStartCursor;
        this._getStartFirstPane = this._getVerticalStartFirstPane;
        this._getCursorStyle = this._getVerticalCursorStyle;
        this.divider.classList.add("vertical");
        this._setSize = this._setVerticalSize;
        break;

      default:
        throw new Error(
          "Please choose either vertical orientation -> 'v' or horizontal -> 'h'.",
        );
    }

    this.divider.addEventListener("mousedown", this._onMouseDown);
    window.addEventListener("mouseup", this._onMouseUp);
    document.addEventListener("mousemove", this._onMouseMove);

    console.log(this);
  }

  _getHorizontalStartCursor(e) {
    return e.x;
  }

  _getVerticalStartCursor(e) {
    return e.y;
  }

  _getHorizontalStartFirstPane() {
    return this.firstPane.offsetHeight;
  }

  _getVerticalStartFirstPane() {
    return this.firstPane.offsetWidth;
  }

  _getHorizontalCursorStyle() {
    return "row-resize";
  }

  _getVerticalCursorStyle() {
    return "col-resize";
  }

  _setVerticalSize(first, second) {
    this.firstPane.style.width = first + "px";
    this.secondPane.style.width = second + "px";
  }
  _setHorizontalSize(first, second) {
    this.firstPane.style.height = first + "px";
    this.secondPane.style.height = second + "px";
  }

  _onMouseDown(e) {
    this.isDragging = true;
    this._startCursor = this._getStartCursor(e);
    this._startFirstPane = this._getStartFirstPane();
    this.divider.classList.add("active");
    document.body.style.cursor = this._getCursorStyle();
  }

  _onMouseMove(e) {
    console.log("movement");
    if (!this.isDragging) return;
    const movement = e.clientY - this._startCursor;

    console.log(movement);
    let newFirstPaneSize = this._startFirstPane + movement;
    const containerRect = this.divider.parent.getBoundingClientRect();
    const minPaneSize = 30;

    // Clamp values
    newFirstPaneSize = Math.max(newFirstPaneSize, minPaneSize);
    newFirstPaneSize = Math.min(
      newFirstPaneSize,
      containerRect.height - minPaneSize - this.divider.offsetHeight,
    );

    this.firstPane.style.flex = "none";
    this.secondPane.style.flex = "none";
    this._setSize(
      newTopHeight,
      containerRect.height - newFirstPaneSize - this.divider.offsetHeight,
    );
  }

  _onMouseUp(e) {
    if (this.isDragging) {
      this.isDragging = false;
      this.divider.classList.remove("active");
      document.body.style.cursor = "";
    }
  }
}

window.Divider = Divider;
