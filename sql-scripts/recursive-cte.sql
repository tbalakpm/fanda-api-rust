with recursive ledger_group_cte as (
	select id, code, group_name, group_desc, group_type, parent_id
	from ledger_groups
	where id = 700
	union all
	select lg.id, lg.code, lg.group_name, lg.group_desc, lg.group_type, lg.parent_id
	from ledger_groups lg
	inner join ledger_group_cte cte on cte.id = lg.parent_id
)
select * from ledger_group_cte;
