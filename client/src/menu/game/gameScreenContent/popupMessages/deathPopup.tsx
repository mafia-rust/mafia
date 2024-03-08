import React from 'react';
import "./settings.css";

export default class DeathPopup {
    render(): React.ReactNode {
        // add correct formatting and translate later
        return(
            <div id = "DeathPopup">
                <h2>"YOU HAVE DIED"</h2>
                <h1>"you can still talk in death chat but you cannot take action or vote</h1>
                <h1>"(Click anywhere to close)"</h1>
            </div>
        );
    }
}