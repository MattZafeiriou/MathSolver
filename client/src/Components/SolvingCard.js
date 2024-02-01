import React from 'react';
import './SolvingCard.css';

class SolvingCard extends React.Component {
    
    constructor(props) {
        super(props);
        this.addEquation = this.addEquation.bind(this);
    }

    addEquation() 
    {
        return (
            <h1><span className='equation'  onClick={
                (e) => {
                    const text = e.currentTarget.textContent;
                    navigator.clipboard.writeText(text);

                    var x = document.getElementById("snackbar");
                    x.className = "show";
                    setTimeout(function(){ x.className = x.className.replace("show", ""); }, 3000);
                }
            }>x + 5 = 0<i class="fa-regular fa-copy"></i></span><span className='explanation'>Explanation</span></h1>
        );
    }

    render() {
        return (
            <>
            <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.0/css/all.min.css"></link>
            <div className='card'>
                <this.addEquation />
                <this.addEquation />
                <this.addEquation />
                <this.addEquation />
            </div>
            <div id="snackbar">Equation copied to clipboard</div> 
            </>
        );
    }
}

export default SolvingCard;