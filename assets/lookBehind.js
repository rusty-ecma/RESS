var runLookBehindAnimation = (function () {
    /**
     * Flag to avoid running more
     * than once
     */
    let running = false;
    /**
     * Set the arrow's fill to "black" at the 
     * provided index
     * @param {number} current index
     */
    function setBlack(idx) {
        if (idx < 0 || idx > 11) {
            return;
        }
        const arrow = document.getElementById(`index-${idx}`);
        arrow.style.fill = 'black';
    }
    /**
     * Set the arrow's fill to "none" at the 
     * provided index
     * @param {number} idx currentIndex
     */
    function setNone(idx) {
        if (idx < 0 || idx > 11) {
            return;
        }
        const arrow = document.getElementById(`index-${idx}`);
        arrow.style.fill = 'none';
    }
    /**
     * Perform the fill setting correctly 
     * - the last 3 are "black"
     * - all others are "none"
     * @param {number} idx current token index
     */
    function updateArrowColors(idx) {
        for (let i = 0; i < 11; i++) {
            if (i < idx - 2 || i > idx) {
                setNone(i);
            } else {
                setBlack(i);
            }
        }
    }
    /**
     * Set all arrow's fill to "none"
     */
    function clearAll() {
        for (let i = 0; i < 11; i++) {
            setNone(i);
        }
        running = false;
    }
    /**
     * Perform one step in the animation
     * 
     * Calling this once will start an async loop
     * for 10 counts finally clearing all arrows
     * @param {number} idx Current iteration count
     */
    function oneTick(idx) {
        if (!idx) idx = 0;
        if (idx > 10) {
            return clearAll();
        }
        updateArrowColors(idx);
        setTimeout(run, 1000, idx + 1)
    }
    /**
     * Exported member, starts the async loop
     * but checks if we are already running
     * and short-circuits if we are
     */
    return function run() {
        if (running) {
            return;
        }
        running = true;
        oneTick();
    }
})();