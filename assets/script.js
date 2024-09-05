const cards = document.querySelectorAll(".card");
cards.forEach(function (card) {
    const expand = card.querySelector(".card-expand");
    card.addEventListener("click", function (event) {
        if (event.target.tagName !== "IMG") {
            expand.classList.toggle("show");
            card.classList.toggle("expanded");
        }
    });
});
