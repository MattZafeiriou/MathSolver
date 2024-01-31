import React from 'react';
import './Algebra.css';
import MathInput from '../../Components/MathInput.js';

class Algebra extends React.Component {
    
    render() {
        return (
            <>
                <div className='algebra'>
                    <h2>Algebra</h2>
                </div>
                <MathInput />
            </>
        );
    }
}

export default Algebra;