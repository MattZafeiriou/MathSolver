import React from 'react';
import './Home.css';

class Home extends React.Component {
    constructor(props) {
        super(props);
        window.history.replaceState(null, null, "/Algebra");
    }
    
    render() {
        return (
            <div>
                <h1>Home</h1>
                <a href="/calculus">Calculus</a>
                <a href="/algebra">Algebra</a>
            </div>
        );
    }
}

export default Home;