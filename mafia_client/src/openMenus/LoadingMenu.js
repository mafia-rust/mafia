import React from "react";
import "../index.css"

export class LoadingMenu extends React.Component {
    constructor(props) {
        super(props);
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    render(){return(<div className="header" style={{height: "100%"}}>
        <h1 className="header-text">{this.props.value?this.props.value:"Loading..."}</h1>
    </div>)}
}