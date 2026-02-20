library ieee;
use ieee.std_logic_1164.all;

entity tb_arrays is
end entity;

architecture sim of tb_arrays is
  signal s_real : real := 0.0;
  signal v_real_arr : real_vector(0 to 3) := (0.0, 0.0, 0.0, 0.0);
  signal v_int_arr : integer_vector(0 to 2) := (0, 0, 0);
begin
  stim : process
  begin
    s_real <= 1.5;
    v_real_arr <= (1.1, 2.2, 3.3, 4.4);
    v_int_arr <= (10, 20, 30);
    wait for 10 ns;

    s_real <= 2.75;
    v_real_arr <= (5.5, 6.6, 7.7, 8.8);
    v_int_arr <= (40, 50, 60);
    wait for 10 ns;

    s_real <= -3.25;
    v_real_arr <= (9.9, 10.1, 11.2, 12.3);
    v_int_arr <= (70, 80, 90);
    wait for 10 ns;

    s_real <= 0.5;
    v_real_arr <= (0.1, 0.2, 0.3, 0.4);
    v_int_arr <= (100, 110, 120);
    wait for 10 ns;

    wait;
  end process;
end architecture;
