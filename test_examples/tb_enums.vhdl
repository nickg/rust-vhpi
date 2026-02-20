library ieee;
use ieee.std_logic_1164.all;

entity tb_enums is
end entity;

architecture sim of tb_enums is
  type state_t is (IDLE, ACTIVE, DONE, ERROR);
  type color_t is (RED, GREEN, BLUE, YELLOW);
  type state_vector_t is array (natural range <>) of state_t;
  type color_vector_t is array (natural range <>) of color_t;

  signal v_state : state_t := IDLE;
  signal v_color : color_t := RED;
  signal v_state_arr : state_vector_t(0 to 2) := (IDLE, IDLE, IDLE);
  signal v_color_arr : color_vector_t(0 to 2) := (RED, RED, RED);
begin
  stim : process
  begin
    v_state <= IDLE;
    v_color <= RED;
    v_state_arr <= (IDLE, ACTIVE, DONE);
    v_color_arr <= (RED, GREEN, BLUE);
    wait for 10 ns;

    v_state <= ACTIVE;
    v_color <= GREEN;
    v_state_arr <= (ACTIVE, DONE, ERROR);
    v_color_arr <= (GREEN, BLUE, YELLOW);
    wait for 10 ns;

    v_state <= DONE;
    v_color <= BLUE;
    v_state_arr <= (DONE, ERROR, IDLE);
    v_color_arr <= (BLUE, YELLOW, RED);
    wait for 10 ns;

    wait;
  end process;
end architecture;
