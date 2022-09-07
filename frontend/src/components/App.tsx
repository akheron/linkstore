import React from 'react'
import { Provider } from 'react-redux'
import { store } from '../store'
import { useLocation } from '../utils/location'
import FrontPage from './FrontPage'
import Create from './Create'
import NotFound from './NotFound'

export default React.memo(function App() {
  const path = useLocation()
  return (
    <div>
      <Provider store={store}>
        {path === '/' ? (
          <FrontPage />
        ) : path === '/new' ? (
          <Create />
        ) : (
          <NotFound />
        )}
      </Provider>
    </div>
  )
})
