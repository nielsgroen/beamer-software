import { createApp } from 'vue';
// import './style.css';
import App from './App.vue';
import PrimeVue from 'primevue/config';
import ToastService from 'primevue/toastservice';
import Tooltip from 'primevue/tooltip';

import InputText from "primevue/inputtext";
import Button from "primevue/button";
import Textarea from 'primevue/textarea';
import Toast from "primevue/toast";

import './css/normalize.css';
// import './css/skeleton.css';
import 'primeflex/primeflex.css';

import 'primevue/resources/themes/saga-blue/theme.css'; //theme
import 'primevue/resources/primevue.min.css'; //core css
import 'primeicons/primeicons.css';
import OrderList from "primevue/orderlist"; //icons



const app = createApp(App);
app.use(PrimeVue);
app.use(ToastService);
app.directive('tooltip', Tooltip);

app.component('InputText', InputText);
app.component('Button', Button);
app.component('Textarea', Textarea);
app.component('OrderList', OrderList);
app.component('Toast', Toast)
app.mount('#app');

