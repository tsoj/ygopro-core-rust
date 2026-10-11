//! Veidos the Dragon of Endless Darkness
use super::prelude::*;

pub const ID: u32 = 8540986;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VEIDOS_ERUPTION_DRAGON], listed_series: &[SET_ASHENED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixRep(c,true,true,s.matfilter,2,99,CARD_VEIDOS_ERUPTION_DRAGON)
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))),
        2,
        99,
        vec![fusion::Mat::Code(CARD_VEIDOS_ERUPTION_DRAGON)],
    );
    // Cannot be destroyed by card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent cannot target it with monster effects
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetValue(function(e,re,rp) return rp==1-e:GetHandlerPlayer() and re:IsMonsterEffect() end)
    effect::set_value_fn(d, e2, lua_fn(e2_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy all Spells and Traps your opponent controls
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e3, Some(condition(e3_condition)));
    // e3:SetTarget(s.stdestg)
    effect::set_target(d, e3, Some(cost_target(stdestg)));
    // e3:SetOperation(s.stdesop)
    effect::set_operation(d, e3, Some(operation(stdesop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy an opponent's card that activated its effect on the field
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.chdescon)
    effect::set_condition(d, e4, Some(condition(chdescon)));
    // e4:SetCost(s.chdescost)
    effect::set_cost(d, e4, Some(cost_target(chdescost)));
    // e4:SetTarget(s.chdestg)
    effect::set_target(d, e4, Some(cost_target(chdestg)));
    // e4:SetOperation(s.chdesop)
    effect::set_operation(d, e4, Some(operation(chdesop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsLevelBelow(9) and c:IsRace(RACE_PYRO,fc,sumtype,tp)
    card::is_level_below(d, c, 9) && card::is_race_ctx(d, c, RACE_PYRO, Some(fc), sumtype, Some(tp))
}
/// `function(e,re,rp) return rp==1-e:GetHandlerPlayer() and re:IsMonsterEffect() end`
fn e2_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.v(1).as_effect().nil();
    let rp = a.p(2);
    let hp = effect::get_handler_player(d, e);
    rp as i64 == 1 - hp as i64 && effect::is_monster_effect(d, re)
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn e3_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.stdestg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn stdestg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
/// `s.stdesop(e,tp,eg,ep,ev,re,r,rp)`
fn stdesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.chdescon(e,tp,eg,ep,ev,re,r,rp)`
fn chdescon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local trig_loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]);
    let trig_loc = info[0].int();
    // return rp==1-tp and trig_loc&LOCATION_ONFIELD>0 and re:GetHandler():IsRelateToEffect(re)
    if a.rp as i64 != 1 - tp as i64 {
        return false;
    }
    if (trig_loc & LOCATION_ONFIELD as i64) <= 0 {
        return false;
    }
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    card::is_relate_to_effect(d, rh, re)
}
/// `s.chdescostfilter(c)`
fn chdescostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ASHENED) and c:IsFaceup() and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_ASHENED as u16]) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.chdescost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chdescost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.chdescostfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&chdescostfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.chdescostfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&chdescostfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.chdestg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chdestg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    true
}
/// `s.chdesop(e,tp,eg,ep,ev,re,r,rp)`
fn chdesop(d: &mut Duel, a: &EffArgs) {
    // if re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) {
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
    }
}
