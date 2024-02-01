import React from 'react';
import './MathInput.css';
import SolvingCard from './SolvingCard.js'
import formatExpression from '../Formatter.js';

class MathInput extends React.Component {
    
    constructor(props) {
        super(props);
        this.type = props.type;
    }

    openMathKeyboard(e) {
        document.querySelector('.input').classList.add('input_keyboard');
        document.querySelector('.keyboard').classList.add('keyboard_active');
        document.querySelector('.cards').classList.add('cards_keyboard');
        e.stopPropagation();
    }

    closeMathKeyboard() {
        document.querySelector('.input').classList.remove('input_keyboard');
        document.querySelector('.keyboard').classList.remove('keyboard_active');
        document.querySelector('.cards').classList.remove('cards_keyboard');
    }

    componentDidMount() {
        document.addEventListener('click', this.closeMathKeyboard);
    }

    submitMath(e) {
        e.preventDefault();
        const math = e.target.previousSibling.value;
        fetch('http://localhost:9000/math?equation=' + formatExpression(math), {
        }).then(res => res.text()).then(data => {
            alert(data);
        });
    }

    render() {
        let placeholder = "Enter a";
        switch (this.type) {
            case "Algebra":
                placeholder += "n algebraic expression";
                break;
            case "Geometry":
                placeholder += " geometric expression";
                break;
            case "Trigonometry":
                placeholder += " trigonometric expression";
                break;
            case "Calculus":
                placeholder += " calculus expression";
                break;
            default:
                break;
        }
        return (
            <>
            <div className='math_input'>
                <div className='type'>
                    <h1>{this.type}</h1>
                </div>
                <div className='cards'>
                    <SolvingCard />
                </div>
                <div className='input'>
                    <form>
                        <input type="text" autoFocus onClick={this.openMathKeyboard} placeholder={placeholder} />
                        <button type="button" onClick={(e) => {
                            this.submitMath(e);
                        }} >Submit</button>
                    </form>
                </div>
                <div className='keyboard' onClick={(e) => {e.stopPropagation();}}>
                    <div className='row'>
                        <h1>+</h1>
                        <h1>-</h1>
                        <h1>*</h1>
                        <h1>/</h1>
                        <h1>x</h1>
                        <h1>x</h1>
                        <h1>x</h1>
                        <h1>x</h1>
                        <h1>x</h1>
                        <h1>x</h1>
                        <h1>k</h1>
                        <h1>ln</h1>
                        <h1>e</h1>
                    </div>
                </div>
            </div>
            </>
        );
    }
}

export default MathInput;
