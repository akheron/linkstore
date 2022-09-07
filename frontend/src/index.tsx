import ReactDOM from 'react-dom/client'
import App from './components/App'
import './index.css'

const domContainer = document.querySelector('#app')
if (domContainer !== null) {
  const root = ReactDOM.createRoot(domContainer)
  root.render(<App />)
}
