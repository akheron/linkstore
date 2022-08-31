import ReactDOM from 'react-dom/client'
import App from './App'

const domContainer = document.querySelector('#app')
if (domContainer !== null) {
  const root = ReactDOM.createRoot(domContainer)
  root.render(<App />)
}
