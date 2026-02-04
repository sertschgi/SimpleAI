class Divider {
  constructor(divider, orientation) {
    if (!(divider instanceof HTMLElement)) {
      throw new Error("Divider constructor requires a DOM container element");
    }
    this.divider = divider;
    this.orientation = orientation;
    this.firstPane = this.divider.previousSibling;
    this.secondPane = this.divider.nextSibling;
    this.parent = this.divider.parentElement;

    this._startFirstPane = null;
    this._startCursor = null;

    this._onMouseDown = this._onMouseDown.bind(this);
    this._onMouseUp = this._onMouseUp.bind(this);
    this._onMouseMove = this._onMouseMove.bind(this);

    switch (orientation) {
      case "h":
        this._getStartCursor = this._getHorizontalStartCursor;
        this._getStartFirstPane = this._getHorizontalStartFirstPane;
        this._getCursorStyle = this._getHorizontalCursorStyle;
        this.divider.classList.add("horizontal");
        this._setSize = this._setHorizontalSize;
        this._getContainerSize = this._getHorizontalContainerSize;
        this._getDividerSize = this._getHorizontalDividerSize;
        break;

      case "v":
        this._getStartCursor = this._getVerticalStartCursor;
        this._getStartFirstPane = this._getVerticalStartFirstPane;
        this._getCursorStyle = this._getVerticalCursorStyle;
        this.divider.classList.add("vertical");
        this._setSize = this._setVerticalSize;
        this._getContainerSize = this._getVerticalContainerSize;
        this._getDividerSize = this._getVerticalDividerSize;
        break;

      default:
        throw new Error(
          "Please choose either vertical orientation -> 'v' or horizontal -> 'h'.",
        );
    }

    this._addListeners();
  }

  _addListeners() {
    this.divider.addEventListener("mousedown", (e) => {
      this._onMouseDown(e);
      window.addEventListener("mousemove", this._onMouseMove);
    });
    window.addEventListener("mouseup", (e) => {
      this._onMouseUp(e);
      window.removeEventListener("mousemove", this._onMouseMove);
    });
  }

  _getHorizontalStartCursor(e) {
    return e.y;
  }

  _getVerticalStartCursor(e) {
    return e.x;
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

  _getHorizontalContainerSize() {
    return this.divider.parentElement.getBoundingClientRect().heigth;
  }

  _getVerticalContainerSize() {
    return this.divider.parentElement.getBoundingClientRect().width;
  }

  _getHorizontalDividerSize() {
    return this.divider.offsetHeight;
  }

  _getVerticalDividerSize() {
    return this.divider.offsetWidth;
  }

  _onMouseMove(e) {
    if (!this.isDragging) return;
    const movement = this._getStartCursor(e) - this._startCursor;

    let newFirstPaneSize = this._startFirstPane + movement;
    const minPaneSize = 300;

    // Clamp values
    newFirstPaneSize = Math.max(newFirstPaneSize, minPaneSize);
    newFirstPaneSize = Math.min(
      newFirstPaneSize,
      this._getContainerSize() - minPaneSize - this._getDividerSize(),
    );

    console.log(newFirstPaneSize);

    this._setSize(
      newFirstPaneSize,
      this._getContainerSize() - newFirstPaneSize - this._getDividerSize(),
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
