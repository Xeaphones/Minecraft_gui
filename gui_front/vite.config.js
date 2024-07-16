import { defineConfig } from 'vite';
import reactRefresh from '@vitejs/plugin-react-refresh';

export default defineConfig({
  plugins: [reactRefresh()],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
      '/ws': {
        target: 'http://localhost:8080',
        ws: true,
      },
      '/getHead': {
        target: 'https://mc-heads.net/avatar/',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/getHead/, ''),
      }
    },
  },
});
