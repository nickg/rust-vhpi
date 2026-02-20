library ieee;
use ieee.std_logic_1164.all;

entity tb_string is
end entity;

architecture sim of tb_string is
    signal v_str : string(1 to 10) := (others => ' ');
    signal v_char : character := 'A';
begin
  stim : process
  begin
    wait for 5 ns;

    v_str <= "Hello     ";
    v_char <= 'H';
    wait for 10 ns;

    v_str <= "TestValue1";
    v_char <= 'T';
    wait for 10 ns;

    v_str <= "VHDL Works";
    v_char <= 'V';
    wait for 10 ns;

    wait;
  end process;
end architecture;
