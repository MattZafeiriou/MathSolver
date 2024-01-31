import React from 'react';
import './MathInput.css';

class MathInput extends React.Component {
    
    constructor(props) {
        super(props);
        this.type = props.type;
    }

    openMathKeyboard(e) {
        document.querySelector('.input').classList.add('input_keyboard');
        document.querySelector('.keyboard').classList.add('keyboard_active');
        e.stopPropagation();
    }

    closeMathKeyboard() {
        document.querySelector('.input').classList.remove('input_keyboard');
        document.querySelector('.keyboard').classList.remove('keyboard_active');
    }

    componentDidMount() {
        document.addEventListener('click', this.closeMathKeyboard);
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
                <div className='input'>
                    <form>
                        <input type="text" autoFocus onClick={this.openMathKeyboard} placeholder={placeholder} />
                        <button type="button" onClick={(e) => {e.stopPropagation();}} >Submit</button>
                    </form>
                </div>
                <div className='keyboard' onClick={(e) => {e.stopPropagation();}}>
                    <h1>Oh hey im a key</h1>
                </div>
            </div>
            </>
        );
    }
}

export default MathInput;