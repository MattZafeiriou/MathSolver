import React from 'react';
import Logo from '../logo.svg';
import './Header.css';

class Header extends React.Component {
    
    render() {
        return (
            <div className='header'>
                <div className='header__logo'>
                    <a href='/'>
                        <img src={Logo} alt='Logo' />
                    </a>
                </div>
                <div className='header__nav'>
                    <a href='/'>Home</a>
                    <a href='/Algebra'>Algebra</a>
                    <a href='/Calculus'>Calculus</a>
                </div>
            </div>
        );
    }
}

export default Header;