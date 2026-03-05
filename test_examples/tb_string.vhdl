library ieee;
use ieee.std_logic_1164.all;

entity tb_string is
end entity;

architecture sim of tb_string is
    type str_array_t is array (natural range <>) of string(1 to 10);
    signal v_str : string(3 to 12) := (others => ' ');
    signal v_char : character := 'A';
    signal v_str_array : str_array_t(6 downto 4) := (others => (others => ' '));
begin
  stim : process
  begin
    wait for 5 ns;

    v_str_array(6) <= "Hello     ";
    v_str <= "Hello     ";
    v_char <= 'H';
    wait for 10 ns;

    v_str_array(5) <= "TestValue1";
    v_str <= "TestValue1";
    v_char <= 'T';
    wait for 10 ns;

    v_str_array(4) <= "VHDL Wörks";
    v_str <= "VHDL Wörks";
    v_char <= 'V';
    wait for 10 ns;

    wait;
  end process;
end architecture;
