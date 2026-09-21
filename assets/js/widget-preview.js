/* ==========================================================================
   Net Flow - Real Widget Screenshot Switcher & Interactions
   ========================================================================== */

(function () {
  'use strict';

  const screenshots = {
    small: '/assets/images/widget-small.png',
    medium: '/assets/images/widget-medium.png',
    large: '/assets/images/widget-large.png',
    board: '/assets/images/widget-board.png'
  };

  // Preload all high-res screenshots for zero-delay switching
  Object.values(screenshots).forEach(src => {
    const img = new Image();
    img.src = src;
  });

  const widgetImg = document.getElementById('widgetScreenshot');
  const sizeBtns = document.querySelectorAll('.size-btn');
  const frame = document.getElementById('screenshotFrame');

  sizeBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      sizeBtns.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');

      const size = btn.dataset.size;
      const targetSrc = screenshots[size];
      if (!targetSrc || !widgetImg) return;

      widgetImg.classList.add('fade-out');

      setTimeout(() => {
        widgetImg.src = targetSrc;

        if (frame) {
          if (size === 'board') {
            frame.style.maxWidth = '580px';
          } else if (size === 'small') {
            frame.style.maxWidth = '360px';
          } else if (size === 'medium') {
            frame.style.maxWidth = '440px';
          } else if (size === 'large') {
            frame.style.maxWidth = '460px';
          }
        }

        widgetImg.onload = () => {
          widgetImg.classList.remove('fade-out');
        };
        setTimeout(() => widgetImg.classList.remove('fade-out'), 50);
      }, 150);
    });
  });

  // Copy Script Handler
  const copyBtn = document.getElementById('copyBtn');
  if (copyBtn) {
    copyBtn.addEventListener('click', () => {
      const code = document.getElementById('installCode').textContent;
      navigator.clipboard.writeText(code).then(() => {
        const originalText = copyBtn.textContent;
        copyBtn.textContent = 'Copied!';
        copyBtn.classList.add('copied');
        setTimeout(() => {
          copyBtn.textContent = originalText;
          copyBtn.classList.remove('copied');
        }, 2000);
      });
    });
  }
})();
