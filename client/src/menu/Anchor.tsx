import React from "react";
import "../index.css";
import "./anchor.css";

type AnchorProps = {
    content: JSX.Element,
    onMount: () => void
}
type AnchorState = {
    mobile: boolean,
    content: JSX.Element,
    errors: Error[]
}

export default class Anchor extends React.Component<AnchorProps, AnchorState> {
    private static instance: Anchor;

    constructor(props: AnchorProps) {
        super(props);

        this.state = {
            mobile: false,
            content: this.props.content,
            errors: []
        }
    }
    
    componentDidMount() {
        Anchor.instance = this;

        window.addEventListener("resize", Anchor.onResize);
        Anchor.onResize();

        this.props.onMount()
    }
    
    private static onResize() {
        Anchor.instance.setState({mobile: window.innerWidth <= 600});
    }
    
    componentWillUnmount() {
        window.removeEventListener("resize", Anchor.onResize);
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
    public static isMobile(): boolean {
        return Anchor.instance.state.mobile;
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
