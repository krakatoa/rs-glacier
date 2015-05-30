var rsGlacier = (function() {
  var init = function() {
    console.log("started rsGlacier");

    $(".settings").on("click", function(e) {
      console.log("bla");
      e.preventDefault()
    });
  }

  return {
    init: init
  }
})();

$(window).load(function () {
  rsGlacier.init();
});
