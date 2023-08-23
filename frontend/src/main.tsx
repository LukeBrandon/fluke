import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router-dom";
import "./styles/index.css";
import { AuthProvider } from 'react-auth-kit'
import ROUTER from "./routes";

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <AuthProvider
            authType="cookie"
            authName="_auth"
            cookieDomain={window.location.hostname}
            cookieSecure={false}
        >
            <RouterProvider router={ROUTES} />
        </AuthProvider>
    </React.StrictMode>,
);
