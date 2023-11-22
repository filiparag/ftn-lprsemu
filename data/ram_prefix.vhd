-------------------------------------------------------
-- Logicko projektovanje racunarskih sistema 1
-- 2011/2012, 2023
--
-- Data RAM
--
-- authors:
-- Ivan Kastelan (ivan.kastelan@rt-rk.com)
-- Milos Subotic (milos.subotic@uns.ac.rs)
-------------------------------------------------------

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity data_ram is
	port (
		iCLK : in std_logic;
		inRST : in std_logic;
		iA : in std_logic_vector(7 downto 0);
		iD : in std_logic_vector(15 downto 0);
		iWE : in std_logic;
		oQ : out std_logic_vector(15 downto 0)
	);
end data_ram;

architecture arch of data_ram is

	type tMEM is array(0 to 255) of std_logic_vector(15 downto 0);
	signal rMEM : tMEM;
	signal sMEM : tMEM := (others => x"0000");

begin

	process (iCLK, inRST)begin
		if inRST = '0' then
			for i in 0 to 255 loop
				rMEM(i) <= sMEM(i);
			end loop;
		elsif rising_edge(iCLK) then
			if iWE = '1' then
				rMEM(to_integer(unsigned(iA))) <= iD;
			end if;
		end if;
	end process;

------------------------------------------------------------------
