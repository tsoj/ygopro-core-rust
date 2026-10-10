//! L・G・D (Five-Headed Link Dragon)
use super::prelude::*;

pub const ID: u32 = 10669138;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,5,5)
    link::add_procedure(d, c, None, 5, Some(5), None, None, None);
    // must link summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.lnklimit)
    effect::set_value_fn(d, e1, aux::lnklimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    // e3:SetValue(s.valcheck)
    effect::set_value_fn(d, e3, effect_card_value(valcheck));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // immune
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e4, EFFECT_IMMUNE_EFFECT);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetValue(s.efilter)
    effect::set_value_fn(d, e4, lua_fn(efilter));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // indes
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_BATTLE);
    // e5:SetValue(s.batfilter)
    effect::set_value_fn(d, e5, lua_fn(batfilter));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // maintenance
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e6, CATEGORY_REMOVE);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e6, EVENT_PHASE + PHASE_END);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetCondition(s.rmcon)
    effect::set_condition(d, e6, Some(condition(rmcon)));
    // e6:SetTarget(s.rmtg)
    effect::set_target(d, e6, Some(cost_target(rmtg)));
    // e6:SetOperation(s.rmop)
    effect::set_operation(d, e6, Some(operation(rmop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial():Filter(Card.IsAttribute,nil,ATTRIBUTE_DARK|ATTRIBUTE_WIND|ATTRIBUTE_FIRE|ATTRIBUTE_WATER|ATTRIBUTE_EARTH)
    let mat = card::get_material(d, c);
    let g = group::filter(d, &mat, &|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK | ATTRIBUTE_WIND | ATTRIBUTE_FIRE | ATTRIBUTE_WATER | ATTRIBUTE_EARTH), NO_EX);
    // if #g==5
    //     and g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_EARTH)
    //     and g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WIND)
    //     and g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_FIRE)
    //     and g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WATER)
    //     and g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_DARK) then
    if g.len() == 5
        && group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_EARTH), 1, NO_EX)
        && group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_WIND), 1, NO_EX)
        && group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_FIRE), 1, NO_EX)
        && group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_WATER), 1, NO_EX)
        && group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK), 1, NO_EX)
    {
        // e:GetLabelObject():SetLabel(1)
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[1]);
    }
    0
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned() and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h) && effect::get_label(d, a.e) == 1
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwner()~=e:GetOwner()
    effect::get_owner(d, a.e(1)) != effect::get_owner(d, a.e(0))
}
/// `s.batfilter(e,c)`
fn batfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK|ATTRIBUTE_WIND|ATTRIBUTE_FIRE|ATTRIBUTE_WATER|ATTRIBUTE_EARTH)
    card::is_attribute(d, a.c(1), ATTRIBUTE_DARK | ATTRIBUTE_WIND | ATTRIBUTE_FIRE | ATTRIBUTE_WATER | ATTRIBUTE_EARTH)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,math.min(#g,5),0,0)
    let n = g.len().min(5) as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g<5 then Duel.SendtoGrave(c,REASON_EFFECT) end
    if g.len() < 5 {
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
    }
    // if #g>=5 then
    if g.len() >= 5 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg=g:Select(tp,5,5,nil)
        let sg = group::select(d, &g, a.tp, 5, 5, false, NO_EX).nil();
        // if #sg~=5 or Duel.Remove(sg,POS_FACEDOWN,REASON_EFFECT)~=5 then
        let removed = sg.len() == 5 && duel::remove(d, sg, POS_FACEDOWN as u8, REASON_EFFECT, None, None) == 5;
        if !removed {
            // Duel.SendtoGrave(c,REASON_EFFECT)
            duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        }
    }
}
