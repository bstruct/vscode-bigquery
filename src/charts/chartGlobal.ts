import { Chart, registerables } from 'chart.js';
Chart.register(...registerables);
(window as any).Chart = Chart;
