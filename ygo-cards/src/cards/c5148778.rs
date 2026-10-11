//! Starving Venom Wing Dragon
//! 覇王暴竜スターヴ・ヴェノム・ウィング・ドラゴン
//! Scripted By The Razgriz
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CLEAR_WING],
    material_setcode: &[SET_CLEAR_WING],
    counter_place_list: &[COUNTER_PREDATOR],
    ..ScriptInfo::DEFAULT
};

pub const ID: u32 = 5148778;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // Fusion Materials: 1 Fusion Monster + 1 "Clear Wing" monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsType,TYPE_FUSION),aux.FilterBoolFunctionEx(Card.IsSetCard,SET_CLEAR_WING))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_FUSION, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_CLEAR_WING as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // Place Predator Counters on monsters Special Summoned to your opponent's field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetOperation(s.counterplaceop)
    effect::set_operation(d, e1, Some(operation(counterplaceop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your opponent Special Summons a monster(s): You can target 1 of them; this card gains ATK equal to its ATK, and its effects are negated.
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.atknegeffcon)
    effect::set_condition(d, e2, Some(condition(atknegeffcon)));
    // e2:SetTarget(s.atknegefftg)
    effect::set_target(d, e2, Some(cost_target(atknegefftg)));
    // e2:SetOperation(s.atknegeffop)
    effect::set_operation(d, e2, Some(operation(atknegeffop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // When another card or effect is activated on the field: You can destroy 1 card on the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:GetHandler()~=e:GetHandler() and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)&LOCATION_ONFIELD>0 end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let re_h = effect::get_handler(d, a.re.nil()).nil();
        let h = effect::get_handler(d, a.e).nil();
        re_h != h && (duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() & LOCATION_ONFIELD as i64) > 0
    })));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Place itself into Pendulum Zone if it is destroyed
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_MZONE) end)
    effect::set_condition(d, e4, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_MZONE)
    })));
    // e4:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e4, Some(cost_target(e4_target)));
    // e4:SetOperation(s.penop)
    effect::set_operation(d, e4, Some(operation(penop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.counterplaceop(e,tp,eg,ep,ev,re,r,rp)`
fn counterplaceop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=eg:Filter(Card.IsControler,nil,1-tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, x| card::is_controler(d, x, 1 - tp), NO_EX);
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // if tc:AddCounter(COUNTER_PREDATOR,1) and tc:IsLevelAbove(2) then
        if card::add_counter(d, tc, COUNTER_PREDATOR as u16, 1, false) && card::is_level_above(d, tc, 2) {
            // It becomes Level 1 as long as it has a Predator Counter
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetCondition(function(e) return e:GetHandler():HasCounter(COUNTER_PREDATOR) end)
            effect::set_condition(d, e1, Some(condition(|d, a| {
                let h = effect::get_handler(d, a.e).nil();
                card::has_counter(d, h, COUNTER_PREDATOR as u16)
            })));
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.atknegefffilter(c,e,tp)`
fn atknegefffilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsCanBeEffectTarget(e) and c:IsLocation(LOCATION_MZONE) and c:HasNonZeroAttack()
    card::is_summon_player(d, c, 1 - tp) && card::is_can_be_effect_target(d, c, Some(e)) && card::is_location(d, c, LOCATION_MZONE) && card::has_non_zero_attack(d, c)
}
/// `s.atknegeffcon(e,tp,eg,ep,ev,re,r,rp)`
fn atknegeffcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.atknegefffilter,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    let (e, tp) = (a.e, a.tp);
    group::is_exists(d, &eg, &|d, c| atknegefffilter(d, c, e, tp), 1, NO_EX)
}
/// `s.atknegefftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atknegefftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    // if chkc then return eg:IsContains(chkc) and s.atknegefffilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&eg, chkc) && atknegefffilter(d, chkc, e, tp);
    }
    // if chk==0 then return eg:IsExists(s.atknegefffilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| atknegefffilter(d, c, e, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local tc=nil
    // if #eg==1 then
    let tc: CardSet = if eg.len() == 1 {
        // tc=eg:GetFirst()
        let t = group::get_first(d, &eg).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, t);
        CardSet::from_card(t)
    } else {
        // tc=eg:FilterSelect(tp,s.atknegefffilter,1,1,nil,e,tp)
        let t = group::filter_select(d, &eg, tp, &|d, c| atknegefffilter(d, c, e, tp), 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, t.clone());
        t
    };
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(tc), 1, 0, 0);
    true
}
/// `s.atknegeffop(e,tp,eg,ep,ev,re,r,rp)`
fn atknegeffop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsFaceup()
    //     and c:UpdateAttack(tc:GetAttack(),RESETS_STANDARD_DISABLE_PHASE_END) and tc:IsNegatableMonster() then
    if card::is_relate_to_effect(d, c, e)
        && card::is_faceup(d, c)
        && card::is_relate_to_effect(d, tc, e)
        && card::is_faceup(d, tc)
        && {
            // c:UpdateAttack(...) returns the change in ATK, a number, which is always true in Lua
            let atk = card::get_attack(d, tc);
            card::update_attack(d, c, atk as i64, Some(RESETS_STANDARD_DISABLE_PHASE_END), None, None);
            true
        }
        && card::is_negatable_monster(d, tc)
    {
        // tc:NegateEffects(c,RESETS_STANDARD_PHASE_END)
        card::negate_effects(d, tc, c, Some(RESETS_STANDARD_PHASE_END), false, None);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD),1,tp,LOCATION_ONFIELD)
    let fg = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(fg), 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// The target of `e4` (`function(e,tp,eg,ep,ev,re,r,rp,chk)`)
fn e4_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsLocation(LOCATION_GRAVE) then Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0) end
    if card::is_location(d, c, LOCATION_GRAVE) {
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    }
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
