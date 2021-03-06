export const STYLING = `
    <style>
        nav.birb-toolbar {
        display: grid;
        grid-template-columns: auto auto;
        grid-gap: 10px;
    }
    nav.birb-toolbar > button {
        font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
        font-weight: 500;
        font-size: 12px;
        line-height: 20px;
        padding: 3px 10px;
        cursor: pointer;
        border-radius: 3px;
        background-image: linear-gradient(-180deg,#fafbfc,#eff3f6 90%);
        border: 1px solid rgba(27,31,35,.2);
        user-select: none; /* supported by Chrome and Opera */
        -webkit-user-select: none; /* Safari */
        -khtml-user-select: none; /* Konqueror HTML */
        -moz-user-select: none; /* Firefox */
        -ms-user-select: none; /* Internet Explorer/Edge */
    }
    nav.birb-toolbar > button:hover {
        background-color: #e6ebf1;
        background-image: linear-gradient(-180deg,#f0f3f6,#e6ebf1 90%);
        background-position: -.5em;
        border-color: rgba(27,31,35,.35);
    }
    nav.birb-toolbar > button:focus {
        box-shadow: 0 0 0 0.2em rgba(3,102,214,.3);
        outline: none;
    }
    nav.birb-toolbar > button:active {
        background-color: #e9ecef;
        background-image: none;
        border-color: rgba(27,31,35,.35);
        box-shadow: inset 0 0.15em 0.3em rgba(27,31,35,.15);
    }
    </style>
`;
