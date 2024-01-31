import React from 'react';
import './MathInput.css';

class MathInput extends React.Component {
    
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
        return (
            <>
            <div className='math_input'>
                <div className='input'>
                    <form>
                        <input type="text" autoFocus onClick={this.openMathKeyboard} />
                        <button type="button" onClick={(e) => {e.stopPropagation();}} >Submit</button>
                    </form>
                </div>
                <div className='keyboard' onClick={(e) => {e.stopPropagation();}}>
                </div>
            </div>
            </>
        );
    }
}

export default MathInput;