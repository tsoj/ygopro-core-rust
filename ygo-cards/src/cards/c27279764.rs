//! Apoqliphort Towers
//! アポクリフォート・キラー
use super::prelude::*;

pub const ID: u32 = 27279764;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_QLI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be Special Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Must be Tribute Summoned by using "Qli" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRIBUTE_LIMIT)
    effect::set_code(d, e2, EFFECT_TRIBUTE_LIMIT);
    // e2:SetValue(s.tlimit)
    effect::set_value_fn(d, e2, effect_card_fn(tlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Summon with 3 tribute
    // local e3=aux.AddNormalSummonProcedure(c,true,false,3,3,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0))
    let _e3 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), None, None);
    // local e4=aux.AddNormalSetProcedure(c,true,false,3,3,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0))
    let _e4 = aux::add_normal_set_procedure(d, c, true, false, Some(3), Some(3), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), None, None);
    // Unaffected by cards' effects
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e5, EFFECT_IMMUNE_EFFECT);
    // e5:SetCondition(function(e) return e:GetHandler():IsNormalSummoned() end)
    effect::set_condition(d, e5, Some(condition(immcon)));
    // e5:SetValue(s.efilter)
    effect::set_value_fn(d, e5, lua_fn(efilter));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Decrease the ATK/DEF of Special Summoned monsters by 500
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e6, EFFECT_UPDATE_ATTACK);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e6, LOCATION_MZONE, LOCATION_MZONE);
    // e6:SetTarget(s.adtg)
    effect::set_target(d, e6, Some(effect_card_fn(adtg)));
    // e6:SetValue(-500)
    effect::set_value(d, e6, -500);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=e6:Clone()
    let e7 = effect::clone(d, e6);
    // e7:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e7, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // Make the opponent send 1 monster to the GY
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e8, aux::stringid(ID, 1));
    // e8:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e8, CATEGORY_TOGRAVE);
    // e8:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e8, EFFECT_TYPE_IGNITION);
    // e8:SetRange(LOCATION_MZONE)
    effect::set_range(d, e8, LOCATION_MZONE);
    // e8:SetCountLimit(1)
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    // e8:SetTarget(s.tgtg)
    effect::set_target(d, e8, Some(cost_target(tgtg)));
    // e8:SetOperation(s.tgop)
    effect::set_operation(d, e8, Some(operation(tgop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
// function s.tlimit(e,c)
fn tlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_QLI)
    !card::is_set_card(d, c, &[SET_QLI as u16])
}
// function s.efilter(e,te)
fn efilter(d: &mut Duel, a: Args) -> bool {
    let (e, te) = (a.e(0), a.e(1));
    // if te:IsSpellTrapEffect() then return true
    if effect::is_spell_trap_effect(d, te) {
        return true;
    }
    // else return aux.qlifilter(e,te) end
    aux::qlifilter().call(d, &[Value::Effect(e), Value::Effect(te)]).truthy()
}
// function(e) return e:GetHandler():IsNormalSummoned() end
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_normal_summoned(d, h)
}
// function s.adtg(e,c)
fn adtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSpecialSummoned()
    card::is_special_summoned(d, c)
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE|LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE | LOCATION_HAND) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,0,LOCATION_MZONE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 0, (LOCATION_MZONE | LOCATION_HAND) as i32);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsMonster,1-tp,LOCATION_MZONE|LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(d, Some(&card::is_monster), 1 - a.tp, LOCATION_MZONE | LOCATION_HAND, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TOGRAVE as u64);
        // local sg=g:Select(1-tp,1,1,nil)
        let sg = group::select(d, &g, 1 - a.tp, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.SendtoGrave(sg,REASON_RULE,PLAYER_NONE,1-tp)
        duel::send_to_grave(d, sg, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp));
    }
}
