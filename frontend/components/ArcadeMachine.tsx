import { chakra } from "@/lib/fonts";

const ArcadeMachine = () => {
  return (
    <div className="container my-10">
      <div className="arcade-machine">
        <div className="shadow"></div>
        <div className="top">
          <div className="stripes"></div>
        </div>
        <div className="screen-container">
          <div className="shadow"></div>
          <div className="screen">
            <div className="screen-display"></div>
            <h2 className={`${chakra.className}`}>GAME SIMULATION</h2>
            <div className="alien-container">
              <div className="alien">
                <div className="ear ear-left"></div>
                <div className="ear ear-right"></div>
                <div className="head-top"></div>
                <div className="head">
                  <div className="eye eye-left"></div>
                  <div className="eye eye-right"></div>
                </div>
                <div className="body"></div>
                <div className="arm arm-left"></div>
                <div className="arm arm-right"></div>
              </div>
            </div>
          </div>
          <div className="joystick">
            <div className="stick"></div>
          </div>
        </div>
        <div className="board">
          <div className="button button-a"></div>
          <div className="button button-b"></div>
          <div className="button button-c"></div>
        </div>
        <div className="bottom">
          <div className="stripes"></div>
        </div>
      </div>
    </div>
  );
};

export default ArcadeMachine;
