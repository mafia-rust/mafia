import React from "react";
import "../index.css"

export class LoadingMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            value: this.props.value ? this.props.value : "Loading..."
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    render(){return(<div className="header" style={{height: "100%"}}>
        <h1 className="header-text">{this.state.value}</h1>
    </div>)}
}