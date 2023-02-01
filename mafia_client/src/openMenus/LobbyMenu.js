import React from "react";

export class LobbyMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }

    renderSettings(){return(<div>
        Settings
    </div>)}
    renderRolePicker(){return(<div>
        Role List
    </div>)}
    renderPlayers(){return(<div>
        Players
    </div>)}

    render(){return(<div>
        {this.state.name}<br/>
        <br/>
        {this.renderSettings()}
        {this.renderRolePicker()}
        {this.renderPlayers()}
        <br/>
        <button style={{width: "90%"}}>Start</button><br/>
    </div>)}
}