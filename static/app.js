// ── ISO 3166-1 numeric → backend country name ─────────────────────────────
const ISO = {
  4:   "Afghanistan",               8:   "Albania",
  12:  "Algeria",                   24:  "Angola",
  32:  "Argentina",                 36:  "Australia",
  40:  "Austria",                   50:  "Bangladesh",
  56:  "Belgium",                   68:  "Bolivia",
  76:  "Brazil",                    100: "Bulgaria",
  104: "Myanmar",                   116: "Cambodia",
  120: "Cameroon",                  124: "Canada",
  144: "Sri Lanka",                 152: "Chile",
  156: "China",                     170: "Colombia",
  178: "Republic of Congo",         180: "Democratic Republic of Congo",
  191: "Croatia",                   192: "Cuba",
  203: "Czech Republic",            208: "Denmark",
  214: "Dominican Republic",        218: "Ecuador",
  231: "Ethiopia",                  246: "Finland",
  250: "France",                    268: "Georgia",
  276: "Germany",                   288: "Ghana",
  300: "Greece",                    320: "Guatemala",
  332: "Haiti",                     340: "Honduras",
  348: "Hungary",                   356: "India",
  360: "Indonesia",                 364: "Iran",
  368: "Iraq",                      376: "Israel",
  380: "Italy",                     388: "Jamaica",
  392: "Japan",                     398: "Kazakhstan",
  400: "Jordan",                    404: "Kenya",
  408: "North Korea",               410: "South Korea",
  414: "Kuwait",                    418: "Laos",
  422: "Lebanon",                   434: "Libya",
  440: "Lithuania",                 458: "Malaysia",
  484: "Mexico",                    496: "Mongolia",
  504: "Morocco",                   508: "Mozambique",
  516: "Namibia",                   524: "Nepal",
  528: "Netherlands",               554: "New Zealand",
  566: "Nigeria",                   578: "Norway",
  586: "Pakistan",                  591: "Panama",
  600: "Paraguay",                  604: "Peru",
  608: "Philippines",               616: "Poland",
  620: "Portugal",                  642: "Romania",
  643: "Russia",                    682: "Saudi Arabia",
  686: "Senegal",                   703: "Slovakia",
  706: "Somalia",                   710: "South Africa",
  724: "Spain",                     729: "Sudan",
  752: "Sweden",                    756: "Switzerland",
  760: "Syria",                     762: "Tajikistan",
  764: "Thailand",                  784: "United Arab Emirates",
  788: "Tunisia",                   792: "Turkey",
  800: "Uganda",                    804: "Ukraine",
  818: "Egypt",                     826: "United Kingdom",
  834: "Tanzania",                  840: "United States",
  858: "Uruguay",                   860: "Uzbekistan",
  862: "Venezuela",                 704: "Vietnam",
  887: "Yemen",                     894: "Zambia",
  716: "Zimbabwe",                  275: "Palestine",
};

// ── Shared state ───────────────────────────────────────────────────────────
let counts         = {};    // country name → article count
let activeEl       = null;  // currently highlighted country element
let refreshMarkers = null;  // set by drawMap once the globe is ready

// ── DOM refs ───────────────────────────────────────────────────────────────
const panel    = document.getElementById("panel");
const backdrop = document.getElementById("backdrop");
const list     = document.getElementById("articles-list");
const tooltip  = document.getElementById("tooltip");

// ═══════════════════════════════════════════════════════════════════════════
// Panel
// ═══════════════════════════════════════════════════════════════════════════

function closePanel() {
  panel.classList.remove("open");
  backdrop.classList.remove("show");
  if (activeEl) {
    activeEl.classList.remove("active");
    activeEl = null;
  }
}

async function openPanel(countryName, el) {
  if (activeEl) activeEl.classList.remove("active");
  activeEl = el;
  el.classList.add("active");

  document.getElementById("panel-country").textContent = countryName;
  const n = counts[countryName] || 0;
  document.getElementById("panel-meta").textContent =
    `${n} article${n !== 1 ? "s" : ""}`;

  list.innerHTML = '<div class="loading"><div class="spinner"></div></div>';
  panel.classList.add("open");
  backdrop.classList.add("show");

  try {
    const res      = await fetch(`/api/news?country=${encodeURIComponent(countryName)}`);
    const articles = await res.json();
    renderArticles(articles);
  } catch {
    list.innerHTML =
      '<div class="empty"><div class="empty-icon">⚠️</div><p>Failed to load articles.</p></div>';
  }
}

function renderArticles(articles) {
  if (!articles.length) {
    list.innerHTML =
      '<div class="empty"><div class="empty-icon">🗞</div><p>No articles for this country yet.</p></div>';
    return;
  }

  list.innerHTML = articles.map(a => `
    <article class="card">
      <div class="card-top">
        <span class="source-tag">${esc(a.source)}</span>
        <span class="card-date">${a.published ? relDate(a.published) : ""}</span>
      </div>
      ${a.url
        ? `<a class="card-title" href="${esc(a.url)}" target="_blank" rel="noopener noreferrer">${esc(a.title)}</a>`
        : `<span class="card-title no-link">${esc(a.title)}</span>`}
      ${a.description ? `<p class="card-summary">${esc(a.description)}</p>` : ""}
    </article>
  `).join("");
}

document.getElementById("close-btn").addEventListener("click", closePanel);
backdrop.addEventListener("click", closePanel);

// ═══════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════

/** Escape HTML special characters to prevent XSS. */
function esc(s) {
  return String(s)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

/** Format a UTC date string as a relative time label. */
function relDate(pub) {
  const m = pub.match(/^(\d{4})-(\d{2})-(\d{2})\s+(\d{2}):(\d{2})/);
  if (!m) return pub;
  const d    = new Date(Date.UTC(+m[1], +m[2] - 1, +m[3], +m[4], +m[5]));
  const diff = Date.now() - d;
  if (diff < 3_600_000)  return `${Math.floor(diff / 60_000)}m ago`;
  if (diff < 86_400_000) return `${Math.floor(diff / 3_600_000)}h ago`;
  return d.toLocaleDateString("en", { month: "short", day: "numeric" });
}

// ═══════════════════════════════════════════════════════════════════════════
// Tooltip
// ═══════════════════════════════════════════════════════════════════════════

function showTooltip(event, name, count) {
  document.getElementById("tip-name").textContent = name;
  const tc = document.getElementById("tip-count");
  if (count != null) {
    tc.textContent  = `${count} article${count !== 1 ? "s" : ""}`;
    tc.style.display = "block";
  } else {
    tc.style.display = "none";
  }
  tooltip.style.opacity = "1";
  moveTooltip(event);
}

function moveTooltip(event) {
  const x  = event.clientX, y  = event.clientY;
  const vw = window.innerWidth, vh = window.innerHeight;
  const tw = 180, th = 50; // estimated tooltip size
  tooltip.style.left = (x + 16 + tw > vw ? x - tw - 8 : x + 16) + "px";
  tooltip.style.top  = (y - th - 8 < 0  ? y + 14      : y - th - 8) + "px";
}

function hideTooltip() {
  tooltip.style.opacity = "0";
}

// ═══════════════════════════════════════════════════════════════════════════
// Globe / Map
// ═══════════════════════════════════════════════════════════════════════════

const INDIA_ISO         = 356;
const ROTATE_SPEED      = 0.12;   // degrees per animation frame
const RESUME_DELAY_MS   = 2500;   // ms before auto-rotation resumes after interaction
const INITIAL_ROTATION  = [0, -25, 0];

async function drawMap(world) {
  const svg = d3.select("#map");
  let W = window.innerWidth;
  let H = window.innerHeight;
  svg.attr("width", W).attr("height", H);

  // ── Ocean gradient ────────────────────────────────────────────────────────
  const defs = svg.append("defs");
  const grad = defs.append("radialGradient")
    .attr("id", "ocean-grad")
    .attr("cx", "38%")
    .attr("cy", "38%");
  grad.append("stop").attr("offset", "0%")  .attr("stop-color", "#0e2a45");
  grad.append("stop").attr("offset", "100%").attr("stop-color", "#040d1a");

  // ── Projection ────────────────────────────────────────────────────────────
  const proj = d3.geoOrthographic()
    .scale(Math.min(W, H) * 0.46)
    .translate([W / 2, H / 2])
    .clipAngle(90)
    .rotate(INITIAL_ROTATION);

  const path = d3.geoPath().projection(proj);
  const g    = svg.append("g");

  // ── Base layers ───────────────────────────────────────────────────────────
  g.append("path")
    .datum({ type: "Sphere" })
    .attr("class", "sphere")
    .style("fill", "url(#ocean-grad)")
    .attr("d", path);

  g.append("path")
    .datum(d3.geoGraticule()())
    .attr("class", "graticule")
    .attr("d", path);

  // ── Countries ─────────────────────────────────────────────────────────────
  const features = topojson.feature(world, world.objects.countries).features;

  g.selectAll(".country")
    .data(features)
    .join("path")
    .attr("class", d => {
      if (+d.id === INDIA_ISO) return "country india-placeholder";
      const name = ISO[+d.id];
      return "country " + (name && counts[name] != null ? "has-news" : "no-news");
    })
    // Hide standard India — replaced by the official boundary below
    .style("display", d => +d.id === INDIA_ISO ? "none" : null)
    .attr("d", path)
    .on("mousemove", function(event, d) {
      const name = ISO[+d.id];
      if (!name) return;
      showTooltip(event, name, counts[name] ?? null);
    })
    .on("mouseleave", hideTooltip)
    .on("click", function(event, d) {
      const name = ISO[+d.id];
      if (name && counts[name] != null) openPanel(name, this);
    })
    .on("dblclick", function(event, d) {
      event.stopPropagation();
      rotateTo(d3.geoCentroid(d));
    });

  g.append("path")
    .datum(topojson.mesh(world, world.objects.countries, (a, b) => a !== b))
    .attr("class", "borders")
    .attr("d", path);

  // ── News markers ──────────────────────────────────────────────────────────
  const markerG = g.append("g").attr("class", "markers");

  function markerRadius(count) {
    return Math.max(10, Math.min(26, 8 + Math.log(count + 1) * 3.2));
  }

  function addMarker(name, count, centroid) {
    const r = markerRadius(count);
    const m = markerG.append("g")
      .attr("class", "marker")
      .style("cursor", "pointer")
      .datum({ name, count, centroid })
      .on("click", function(event) {
        event.stopPropagation();
        if (counts[name] != null) openPanel(name, this);
      });

    m.append("circle")
      .attr("r", r)
      .attr("fill", "rgba(29,78,216,0.82)")
      .attr("stroke", "rgba(255,255,255,0.35)")
      .attr("stroke-width", 1.2);

    m.append("text")
      .attr("text-anchor", "middle")
      .attr("dominant-baseline", "central")
      .attr("font-size", r < 14 ? "8px" : "10px")
      .text(count > 999 ? (Math.round(count / 100) / 10) + "k" : count);
  }

  // Build centroid lookup so refreshMarkers can add new-country markers later
  const centroidOf = {};
  for (const feat of features) {
    const name = ISO[+feat.id];
    if (name) centroidOf[name] = d3.geoCentroid(feat);
  }

  for (const name of Object.keys(centroidOf)) {
    if (counts[name] == null) continue;
    addMarker(name, counts[name], centroidOf[name]);
  }

  // Called after every loadCounts() to sync marker numbers with fresh data
  refreshMarkers = function() {
    // Update existing markers
    markerG.selectAll(".marker").each(function(d) {
      const newCount = counts[d.name];
      if (newCount == null) {
        // Country lost all articles — hide marker
        d3.select(this).style("display", "none");
        return;
      }
      if (newCount === d.count) return; // unchanged
      d.count = newCount;
      const r = markerRadius(newCount);
      d3.select(this).select("circle").attr("r", r);
      d3.select(this).select("text")
        .attr("font-size", r < 14 ? "8px" : "10px")
        .text(newCount > 999 ? (Math.round(newCount / 100) / 10) + "k" : newCount);
    });

    // Add markers for countries that are new since page load
    const existing = new Set(
      markerG.selectAll(".marker").data().map(d => d.name)
    );
    for (const [name, count] of Object.entries(counts)) {
      if (!existing.has(name) && centroidOf[name]) {
        addMarker(name, count, centroidOf[name]);
      }
    }
  };

  // ── Redraw ────────────────────────────────────────────────────────────────
  function redraw() {
    g.selectAll("path").attr("d", path);

    const [λ, φ]    = proj.rotate();
    const frontCenter = [-λ, -φ];

    markerG.selectAll(".marker").each(function(d) {
      const onFront = d3.geoDistance(d.centroid, frontCenter) < Math.PI / 2 - 0.1;
      if (!onFront) { d3.select(this).style("display", "none"); return; }
      const pt = proj(d.centroid);
      if (!pt)      { d3.select(this).style("display", "none"); return; }
      d3.select(this)
        .style("display", null)
        .attr("transform", `translate(${pt[0].toFixed(1)},${pt[1].toFixed(1)})`);
    });
  }

  // ── Auto-rotation ─────────────────────────────────────────────────────────
  let autoRotate   = true;
  let resumeTimer  = null;

  function pauseAndResume() {
    autoRotate = false;
    clearTimeout(resumeTimer);
    resumeTimer = setTimeout(() => { autoRotate = true; }, RESUME_DELAY_MS);
  }

  d3.timer(() => {
    if (!autoRotate) return;
    const [λ, φ, γ] = proj.rotate();
    proj.rotate([λ + ROTATE_SPEED, φ, γ]);
    redraw();
  });

  // ── Drag to rotate ────────────────────────────────────────────────────────
  let dragOrigin   = null;
  let rotateOrigin = null;

  svg.call(d3.drag()
    .on("start", function(event) {
      autoRotate  = false;
      clearTimeout(resumeTimer);
      dragOrigin   = [event.x, event.y];
      rotateOrigin = proj.rotate();
      svg.classed("dragging", true);
      hideTooltip();
    })
    .on("drag", function(event) {
      const dx   = event.x - dragOrigin[0];
      const dy   = event.y - dragOrigin[1];
      const sens = 90 / proj.scale();
      proj.rotate([
        rotateOrigin[0] + dx * sens,
        Math.max(-60, Math.min(60, rotateOrigin[1] - dy * sens)),
        rotateOrigin[2],
      ]);
      redraw();
    })
    .on("end", function() {
      svg.classed("dragging", false);
      pauseAndResume();
    })
  );

  // ── Smooth fly-to ─────────────────────────────────────────────────────────
  function rotateTo([lon, lat]) {
    autoRotate = false;
    clearTimeout(resumeTimer);
    const start  = proj.rotate();
    const end    = [-lon, -lat, 0];
    const interp = d3.interpolate(start, end);
    d3.transition()
      .duration(750)
      .ease(d3.easeCubicInOut)
      .tween("rotate", () => t => { proj.rotate(interp(t)); redraw(); })
      .on("end", pauseAndResume);
  }

  // Double-click on ocean → reset to default rotation
  svg.on("dblclick", () => rotateTo([0, 0]));

  // ── Zoom (wheel + buttons) ────────────────────────────────────────────────
  const minScale = Math.min(W, H) * 0.2;
  const maxScale = Math.min(W, H) * 1.5;

  function clampScale(s) {
    return Math.max(minScale, Math.min(maxScale, s));
  }
  function scaleBy(factor) {
    proj.scale(clampScale(proj.scale() * factor));
    redraw();
  }

  svg.on("wheel", function(event) {
    event.preventDefault();
    scaleBy(event.deltaY < 0 ? 1.12 : 0.89);
  }, { passive: false });

  document.getElementById("btn-zoom-in") .addEventListener("click", () => scaleBy(1.4));
  document.getElementById("btn-zoom-out").addEventListener("click", () => scaleBy(0.714));
  document.getElementById("btn-reset")   .addEventListener("click", () => {
    proj.scale(Math.min(W, H) * 0.46).rotate(INITIAL_ROTATION);
    redraw();
  });

  // ── Drag-distance guard ───────────────────────────────────────────────────
  // Prevents a drag gesture from also firing a click event.
  let mouseDownPos = null;
  svg.on("mousedown.guard", e => { mouseDownPos = [e.clientX, e.clientY]; });
  svg.on("click.guard", e => {
    if (mouseDownPos) {
      const dx = e.clientX - mouseDownPos[0];
      const dy = e.clientY - mouseDownPos[1];
      if (dx * dx + dy * dy > 25) e.stopImmediatePropagation();
    }
    mouseDownPos = null;
  }, true);

  // ── Responsive resize ─────────────────────────────────────────────────────
  window.addEventListener("resize", () => {
    W = window.innerWidth;
    H = window.innerHeight;
    svg.attr("width", W).attr("height", H);
    proj.translate([W / 2, H / 2]).scale(Math.min(W, H) * 0.46);
    redraw();
  });

  // ── Official India boundary (Survey of India / GoI claim line) ────────────
  return fetch("/india-official.geojson")
    .then(r => r.json())
    .then(indiaGeo => {
      g.append("path")
        .datum(indiaGeo)
        .attr("class", "country " + (counts["India"] != null ? "has-news" : "no-news"))
        .attr("d", path)
        .attr("id", "india-official")
        .on("mousemove", event => showTooltip(event, "India", counts["India"] ?? null))
        .on("mouseleave", hideTooltip)
        .on("click", function() {
          if (counts["India"] != null) openPanel("India", this);
        })
        .on("dblclick", function(event) {
          event.stopPropagation();
          rotateTo(d3.geoCentroid(indiaGeo));
        });

      window._indiaOfficialEl = document.getElementById("india-official");

      // India path was appended after markerG — re-raise markers to the top
      // so they render above all country paths.
      g.node().appendChild(markerG.node());
    })
    .catch(() => {});
}

// ═══════════════════════════════════════════════════════════════════════════
// Refresh country colours after a data reload
// ═══════════════════════════════════════════════════════════════════════════

function refreshClasses() {
  d3.selectAll(".country").attr("class", function(d) {
    if (+d.id === INDIA_ISO) return "country india-placeholder";
    const name    = ISO[+d.id];
    const hasNews = name && counts[name] != null;
    const isActive = this === activeEl;
    return `country ${hasNews ? "has-news" : "no-news"}${isActive ? " active" : ""}`;
  });

  if (window._indiaOfficialEl) {
    const hasNews  = counts["India"] != null;
    const isActive = window._indiaOfficialEl === activeEl;
    window._indiaOfficialEl.setAttribute(
      "class",
      `country ${hasNews ? "has-news" : "no-news"}${isActive ? " active" : ""}`,
    );
  }
}

// ═══════════════════════════════════════════════════════════════════════════
// Data loading
// ═══════════════════════════════════════════════════════════════════════════

async function loadCounts() {
  const data = await fetch("/api/countries").then(r => r.json());
  counts = {};
  for (const c of data.countries) counts[c.name] = c.count;

  const total = data.countries.reduce((sum, c) => sum + c.count, 0);
  document.getElementById("status").innerHTML =
    `<span class="dot"></span>${total.toLocaleString()} articles &nbsp;·&nbsp; ${esc(data.updated)}`;

  return data;
}

// ═══════════════════════════════════════════════════════════════════════════
// Bootstrap
// ═══════════════════════════════════════════════════════════════════════════

async function init() {
  try {
    const [world] = await Promise.all([
      d3.json("/countries-110m.json"),
      loadCounts(),
    ]);
    await drawMap(world);
  } catch (err) {
    console.error(err);
    document.getElementById("status").textContent = "⚠ Failed to load";
  }

  // Refresh article counts every 5 minutes
  setInterval(async () => {
    await loadCounts();
    refreshClasses();
    if (refreshMarkers) refreshMarkers();
  }, 5 * 60 * 1000);
}

init();
