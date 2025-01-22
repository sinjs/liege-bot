export function isDesignMode() {
  return localStorage.getItem("DesignMode") === "true";
}

function setDesignMode(enable: unknown) {
  if (typeof enable !== "boolean")
    throw new TypeError("Usage: setDesignMode(enable: boolean)");
  localStorage.setItem("DesignMode", String(enable));
  location.reload();
}

const dev = {
  isDesignMode,
  setDesignMode,
};

export function initDev() {
  Object.freeze(dev);
  Object.defineProperty(window, "dev", { value: dev });
}
