class VNode {
  constructor(x, y, label, params) {
    this.x = x;
    this.y = y;
    this.label = label;
    // params: [{ type: "input" | "output", name: "param" }];
    this.inputs = params.filter((p) => p.type === "input");
    this.outputs = params.filter((p) => p.type === "output");
    this.paramSpacing = 32;
    this.width = 160;
    this.height = Math.max(
      48,
      32 +
        Math.max(this.inputs.length, this.outputs.length) * this.paramSpacing,
    );
  }
  getInputCoords(i) {
    return {
      x: this.x,
      y: this.y + 32 + i * this.paramSpacing,
    };
  }
  getOutputCoords(i) {
    return {
      x: this.x + this.width,
      y: this.y + 32 + i * this.paramSpacing,
    };
  }
  contains(px, py) {
    return (
      px > this.x &&
      px < this.x + this.width &&
      py > this.y &&
      py < this.y + this.height
    );
  }
  inputHit(px, py) {
    for (let i = 0; i < this.inputs.length; ++i) {
      let p = this.getInputCoords(i);
      if (Math.hypot(px - p.x, py - p.y) < 14) return i;
    }
    return null;
  }
  outputHit(px, py) {
    for (let i = 0; i < this.outputs.length; ++i) {
      let p = this.getOutputCoords(i);
      if (Math.hypot(px - p.x, py - p.y) < 14) return i;
    }
    return null;
  }
}
class Connection {
  constructor(fromNode, fromOutput, toNode, toInput) {
    this.fromNode = fromNode;
    this.fromOutput = fromOutput;
    this.toNode = toNode;
    this.toInput = toInput;
  }
}

const canvas = document.getElementById("draw");
const viewport = document.getElementById("viewport");
const ctx = canvas.getContext("2d");

const gridSpacing = 40;
const dotRadius = 2;
let zoom = 1;
let offsetX = 0,
  offsetY = 0;

let nodes = [
  new VNode(100, 100, "A", [{ type: "output", name: "outA" }]),
  new VNode(400, 200, "B", [{ type: "input", name: "inB" }]),
  new VNode(200, 320, "C", [
    { type: "input", name: "inC" },
    { type: "output", name: "outC" },
    { type: "output", name: "outC2" },
  ]),
];

let connections = [];
let mouse = { x: 0, y: 0 };
let draggingNode = null,
  dragOffsetX,
  dragOffsetY;
let connectingFrom = null; // { node, outIdx }
let isPanning = false;
let panStart = { x: 0, y: 0 },
  panOrigin = { x: 0, y: 0 };

function fitNodeSizeToGrid(node) {
  node.width = Math.ceil(node.width / gridSpacing) * gridSpacing;
  node.height = Math.ceil(node.height / gridSpacing) * gridSpacing;
}

function resize() {
  const rect = viewport.getBoundingClientRect();
  canvas.width = rect.width;
  canvas.height = rect.height;
  draw();
}
window.addEventListener("resize", resize);

function toEditor(x, y) {
  return {
    x: (x - offsetX) / zoom,
    y: (y - offsetY) / zoom,
  };
}

function draw() {
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.setTransform(zoom, 0, 0, zoom, offsetX, offsetY);

  // Grid
  ctx.fillStyle = "#3a3c40";
  const [w, h] = [
    (canvas.width - offsetX) / zoom,
    (canvas.height - offsetY) / zoom,
  ];
  const startX = Math.floor(-offsetX / zoom / gridSpacing) * gridSpacing;
  const startY = Math.floor(-offsetY / zoom / gridSpacing) * gridSpacing;
  for (let x = startX; x < w + gridSpacing; x += gridSpacing)
    for (let y = startY; y < h + gridSpacing; y += gridSpacing) {
      ctx.beginPath();
      ctx.arc(x, y, dotRadius, 0, 2 * Math.PI);
      ctx.fill();
    }

  // Connections
  for (let c of connections) {
    const from = c.fromNode.getOutputCoords(c.fromOutput);
    const to = c.toNode.getInputCoords(c.toInput);
    ctx.strokeStyle = "#fb0";
    ctx.lineWidth = 3;
    ctx.beginPath();
    ctx.moveTo(from.x, from.y);
    ctx.bezierCurveTo(from.x + 40, from.y, to.x - 40, to.y, to.x, to.y);
    ctx.stroke();
  }
  // Live connect preview
  if (connectingFrom) {
    const from = connectingFrom.node.getOutputCoords(connectingFrom.outIdx);
    ctx.strokeStyle = "#fb0";
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(from.x, from.y);
    ctx.bezierCurveTo(
      from.x + 40,
      from.y,
      mouse.x - 40,
      mouse.y,
      mouse.x,
      mouse.y,
    );
    ctx.stroke();
  }

  // Nodes
  for (let n of nodes) {
    fitNodeSizeToGrid(n);

    ctx.fillStyle = "#374565";
    ctx.strokeStyle = "#58a";
    ctx.lineWidth = 2;
    ctx.fillRect(n.x, n.y, n.width, n.height);
    ctx.strokeRect(n.x, n.y, n.width, n.height);
    ctx.fillStyle = "#fff";
    ctx.font = "16px sans-serif";
    ctx.fillText(n.label, n.x + 10, n.y + 22);

    // Inputs (left)
    n.inputs.forEach((input, i) => {
      const p = n.getInputCoords(i);
      ctx.beginPath();
      ctx.arc(p.x, p.y, 12, 0, 2 * Math.PI);
      ctx.fillStyle = "#48e";
      ctx.fill();
      ctx.stroke();
      ctx.fillStyle = "#fff";
      ctx.fillText(input.name, p.x + 16, p.y + 6);
    });

    // Outputs (right)
    n.outputs.forEach((output, i) => {
      const p = n.getOutputCoords(i);
      ctx.beginPath();
      ctx.arc(p.x, p.y, 12, 0, 2 * Math.PI);
      ctx.fillStyle = "#fa3";
      ctx.fill();
      ctx.stroke();
      ctx.fillStyle = "#fff";
      ctx.fillText(output.name, p.x - 60, p.y + 6);
    });
  }

  ctx.setTransform(1, 0, 0, 1, 0, 0);
}

function add_listeners() {
  viewport.addEventListener("mousedown", (e) => {
    console.log("FIRED!");
    const p = toEditor(e.offsetX, e.offsetY);
    mouse = p;
    // output hit first (for drag to input)
    for (let node of nodes) {
      let outIdx = node.outputHit(p.x, p.y);
      if (outIdx !== null) {
        connectingFrom = { node, outIdx };
        return;
      }
    }
    // hit test nodes
    for (let node of nodes) {
      if (node.contains(p.x, p.y)) {
        draggingNode = node;
        dragOffsetX = p.x - node.x;
        dragOffsetY = p.y - node.y;
        return;
      }
    }
    // Panning
    isPanning = true;
    panOrigin.x = offsetX;
    panOrigin.y = offsetY;
    panStart.x = e.offsetX;
    panStart.y = e.offsetY;
  });

  viewport.addEventListener("mousemove", (e) => {
    const p = toEditor(e.offsetX, e.offsetY);
    mouse = p;
    if (draggingNode) {
      let newX = Math.round((p.x - dragOffsetX) / gridSpacing) * gridSpacing;
      let newY = Math.round((p.y - dragOffsetY) / gridSpacing) * gridSpacing;
      draggingNode.x = newX;
      draggingNode.y = newY;
      fitNodeSizeToGrid(draggingNode);
      draw();
    } else if (isPanning) {
      offsetX = panOrigin.x + (e.offsetX - panStart.x);
      offsetY = panOrigin.y + (e.offsetY - panStart.y);
      draw();
    } else if (connectingFrom) {
      draw();
    }
  });

  viewport.addEventListener("mouseup", (e) => {
    const p = toEditor(e.offsetX, e.offsetY);
    if (draggingNode) {
      draggingNode = null;
      draw();
    } else if (connectingFrom) {
      for (let node of nodes) {
        let inIdx = node.inputHit(p.x, p.y);
        if (inIdx !== null && node !== connectingFrom.node) {
          connections.push(
            new Connection(
              connectingFrom.node,
              connectingFrom.outIdx,
              node,
              inIdx,
            ),
          );
          break;
        }
      }
      connectingFrom = null;
      draw();
    } else if (isPanning) {
      isPanning = false;
      draw();
    }
  });

  viewport.addEventListener("wheel", (e) => {
    let scale = 1 + (e.deltaY < 0 ? 0.1 : -0.1);
    let mx = e.offsetX,
      my = e.offsetY;
    const before = toEditor(mx, my);
    zoom = Math.max(0.4, Math.min(2.5, zoom * scale));
    const after = toEditor(mx, my);
    offsetX += (after.x - before.x) * zoom;
    offsetY += (after.y - before.y) * zoom;
    draw();
    e.preventDefault();
  });
}

function run() {
  console.log("RUNNING");
  add_listeners();
  resize();
  draw();
}

window.run_editor = run;
window.run_editor();
