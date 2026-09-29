const svg = document.querySelector<SVGSVGElement>("#map");

if (!svg) {
    throw new Error("Could not find #map");
}
