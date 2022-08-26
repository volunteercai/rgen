package {{package_name}}.entity;

public class {{entity.name}} {

	{% for filed in entity.fileds %}private {{filed.type_}} {{filed.name}};
	
	{% endfor %}
}