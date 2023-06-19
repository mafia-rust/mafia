import React from "react";
import "../index.css";
import "./anchor.css";

type AnchorProps = {
    content: JSX.Element,
    onMount: () => void
}
type AnchorState = {
    content: JSX.Element,
    errors: Error[]
}

export default class Anchor extends React.Component<AnchorProps, AnchorState> {
    private static instance: Anchor;

    constructor(props: AnchorProps) {
        super(props);

        this.state = {
            content: this.props.content,
            errors: []
        }
    }
    componentDidMount() {
        Anchor.instance = this;

        this.props.onMount()
    }
    render(){
        return <div className="anchor">
            {this.state.content}
            {this.state.errors.map((error, index) => {
                return <ErrorCard 
                    key={index}
                    onClose={() => this.setState({ 
                        errors: this.state.errors.slice(0, index).concat(this.state.errors.slice(index+1)) 
                    })}
                    error={error}
                />;
            })}
        </div>
    }

    public static setContent(content: JSX.Element){
        Anchor.instance.setState({content : content});
    }
    public static queueError(title: string, body: string) {
        Anchor.instance.setState({errors: [{title, body}, ...Anchor.instance.state.errors]});
    }
}

interface Error {
    title: string,
    body: string
}

function ErrorCard(props: { error: Error, onClose: () => void }) {
    return <div className="errorCard slide-in">
        <header>
            {props.error.title}
        </header>
        <button onClick={() => props.onClose()}>
            ✕
        </button>
        <div>
            {props.error.body}
        </div>
    </div>
}
