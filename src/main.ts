import { createApp } from 'vue';
import { createPinia } from "pinia";
// import './style.css';
import App from './App.vue';
import PrimeVue from 'primevue/config';
import ToastService from 'primevue/toastservice';
import Tooltip from 'primevue/tooltip';

import InputText from "primevue/inputtext";
import Button from "primevue/button";
import Textarea from 'primevue/textarea';
import Toast from "primevue/toast";
import Panel from "primevue/panel";
import Sidebar from "primevue/sidebar";

import './css/normalize.css';
// import './css/skeleton.css';
import 'primeflex/primeflex.css';

import 'primevue/resources/themes/saga-blue/theme.css'; //theme
import 'primevue/resources/primevue.min.css'; //core css
import 'primeicons/primeicons.css';
import OrderList from "primevue/orderlist"; //icons



const app = createApp(App);
const pinia = createPinia();
app.use(PrimeVue);
app.use(ToastService);
app.use(pinia);
app.directive('tooltip', Tooltip);

app.component('InputText', InputText);
app.component('Button', Button);
app.component('Textarea', Textarea);
app.component('OrderList', OrderList);
app.component('Toast', Toast);
app.component('Panel', Panel);
app.component('Sidebar', Sidebar);
app.mount('#app');

