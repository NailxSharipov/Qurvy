<style>

#canvas {
    display: block;
    border: 2px dotted #80808080;
}
</style>

<script type="text/javascript">
window.addEventListener('DOMContentLoaded', function() {
  const paths = [
    '../js/approximation/editor.js',
    './js/approximation/editor.js',
    '/js/approximation/editor.js'
  ];
  async function fileExists(path) {
    try {
      const response = await fetch(path, { method: 'HEAD' });
      return response.ok;
    } catch (e) {
      return false;
    }
  }
  (async () => {
    for (const path of paths) {
      if (await fileExists(path)) {
        const script = document.createElement('script');
        script.src = path;
        document.head.appendChild(script);
        break;
      }
    }
  })();
});
</script>

## Approximation
<p style="font-style: italic;">Click on the canvas to drag the points</p>
<canvas id="canvas" width="1000" height="1000"></canvas>