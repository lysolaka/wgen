fetch("/sidebar.html")
  .then(response => response.text())
  .then(html => {
    // Draw the sidebar
    document.getElementById("sidebar").innerHTML = html;

    // Clean up the path
    const current = window.location.pathname;

    document.querySelectorAll("#sidebar a").forEach(link => {
      if (link.getAttribute("href") === current) {
        // Highlight the currently open item
        link.classList.add("active");

        // Expand its section
        const section = link.closest(".section");
        section?.classList.add("open");

        // Expand its subsection
        const subsection = link.closest(".subsection");
        subsection?.classList.add("open");

        // Expand its section button
        const button = section?.querySelector(".sec-header").querySelector(".sec-button")
        button?.classList.add("open");
      }
    });
  });

// Toggle the list closest to the button
function toggleList(button, match) {
  button.closest(match).classList.toggle("open");
  button.classList.toggle("open");
}
