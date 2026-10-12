//! Dark Paladin - Wave-Motion
//! 波動の超魔導剣士－ブラック・パラディン
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 100459009;

pub const INFO: ScriptInfo = ScriptInfo {
    material_setcode: &[SET_DARK_MAGICIAN],
    listed_names: &[CARD_DARK_MAGICIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Dark Magician" + 1 Level 7 or higher monster
    // Fusion.AddProcMix(c,true,true,CARD_DARK_MAGICIAN,aux.FilterBoolFunctionEx(Card.IsLevelAbove,7))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_DARK_MAGICIAN),
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::is_level_above(d, c, 7))),
        ],
    );
    // c:AddMustBeFusionSummoned()
    card::add_must_be_fusion_summoned(d, c);
    // Gains 500 ATK for every other monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return 500*Duel.GetMatchingGroupCount(nil,0,LOCATION_MZONE,LOCATION_MZONE,c) end)
    effect::set_value_fn(d, e1, lua_fn(atk_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can attack all your opponent's monsters, once each
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e2, EFFECT_ATTACK_ALL);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn, when your opponent activates a Spell Card or effect (Quick Effect): You can negate that effect, then you can destroy 1 card on the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and re:IsSpellEffect() and Duel.IsChainDisablable(ev) end)
    effect::set_condition(d, e3, Some(condition(e3_condition)));
    // e3:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e3, Some(cost_target(e3_target)));
    // e3:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e3, Some(operation(e3_operation)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // After this card destroys a card by battle or card effect and sends it to the GY, neither player can activate its effects while it is in the GY
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetRange(LOCATION_ALL)
    effect::set_range(d, e4, LOCATION_ALL);
    // e4:SetOperation(s.cannotactop)
    effect::set_operation(d, e4, Some(operation(cannotactop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}

/// `function(e,c) return 500*Duel.GetMatchingGroupCount(nil,0,LOCATION_MZONE,LOCATION_MZONE,c) end`
fn atk_value(d: &mut Duel, a: Args) -> i64 {
    let ex = match a.card(1) {
        Some(c) => c.into(),
        None => NO_EX,
    };
    500 * duel::get_matching_group_count(d, None, 0, LOCATION_MZONE, LOCATION_MZONE, ex) as i64
}

/// The condition of e3: `rp==1-tp and re:IsSpellEffect() and Duel.IsChainDisablable(ev)`
fn e3_condition(d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp && effect::is_spell_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
}

/// The target of e3: `if chk==0 then return true end ...`
fn e3_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}

/// The operation of e3: `function(e,tp,eg,ep,ev,re,r,rp)`
fn e3_operation(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateEffect(ev) then
    if duel::negate_effect(d, a.ev as u8) {
        // local g=Duel.GetFieldGroup(0,LOCATION_ONFIELD,LOCATION_ONFIELD)
        let g = duel::get_field_group(d, 0, LOCATION_ONFIELD, LOCATION_ONFIELD);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local dg=g:Select(tp,1,1,nil)
            let dg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(dg)
            duel::hint_selection(d, dg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}

/// `s.cannotactfilter(c,rc,re_chk)`
fn cannotactfilter(d: &mut Duel, c: CardId, rc: CardId, re_chk: bool) -> bool {
    // return (rc:IsReasonCard(c) or re_chk) and c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsLocation(LOCATION_GRAVE)
    (card::is_reason_card(d, rc, c) || re_chk) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && card::is_location(d, c, LOCATION_GRAVE)
}

/// `s.cannotactop(e,tp,eg,ep,ev,re,r,rp)`
fn cannotactop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local re_chk=re and re:GetHandler()==c
    let re_chk = match a.re {
        Some(re) => effect::get_handler(d, re) == Some(c),
        None => false,
    };
    // local g=eg:Filter(s.cannotactfilter,nil,c,re_chk)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, x| cannotactfilter(d, x, c, re_chk), NO_EX);
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_EXC_GRAVE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
