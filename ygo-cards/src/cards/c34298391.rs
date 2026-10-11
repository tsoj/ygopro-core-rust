//! Mind Scan
use super::prelude::*;

pub const ID: u32 = 34298391;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_TOON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e0, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // While you have a "Toon" card in your field or GY, your opponent must keep their hand revealed, also you can look at their Set cards at any time
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetCode(EFFECT_PUBLIC)
    effect::set_code(d, e1, EFFECT_PUBLIC);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_TOON),e:GetHandlerPlayer(),LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let p = effect::get_handler_player(d, a.e);
            duel::is_existing_matching_card(
                d,
                Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_TOON as u16]))),
                p,
                LOCATION_ONFIELD | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            )
        })),
    );
    // e1:SetTargetRange(0,LOCATION_HAND|LOCATION_ONFIELD)
    effect::set_target_range(d, e1, 0, LOCATION_HAND | LOCATION_ONFIELD);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you have a Toon monster and a "Toon" Spell in your field and/or GY: You can declare 1 card name that is not among the cards/effects activated in this Chain; negate the activated effects of cards with that original name until the end of this turn, while this card is face-up on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.declcon)
    effect::set_condition(d, e2, Some(condition(declcon)));
    // e2:SetTarget(s.decltg)
    effect::set_target(d, e2, Some(cost_target(decltg)));
    // e2:SetOperation(s.declop)
    effect::set_operation(d, e2, Some(operation(declop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.declconfilter(c)`
fn declconfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsType(TYPE_TOON) and c:IsMonster()) or (c:IsSetCard(SET_TOON) and c:IsSpell())) and c:IsFaceup()
    ((card::is_type(d, c, TYPE_TOON) && card::is_monster(d, c))
        || (card::is_set_card(d, c, &[SET_TOON as u16]) && card::is_spell(d, c)))
        && card::is_faceup(d, c)
}
/// `s.declcon(e,tp,eg,ep,ev,re,r,rp)`
fn declcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroup(s.declconfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,nil):GetClassCount(Card.GetMainCardType)==2
    let g = duel::get_matching_group(d, Some(&declconfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_main_card_type(d, c) as i64) == 2
}
/// `s.decltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn decltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local current_chain=Duel.GetCurrentChain()
    let current_chain = duel::get_current_chain(d, false);
    // s.announce_filter={TYPE_NORMAL,OPCODE_ISTYPE,OPCODE_NOT}
    let mut announce_filter: Vec<u64> = vec![TYPE_NORMAL as u64, OPCODE_ISTYPE, OPCODE_NOT];
    // if current_chain>1 then
    if current_chain > 1 {
        // for i=1,current_chain-1 do
        for i in 1..current_chain {
            // local trig_code1,trig_code2=Duel.GetChainInfo(i,CHAININFO_TRIGGERING_CODE,CHAININFO_TRIGGERING_CODE2)
            let info = duel::get_chain_info(d, i as u8, &[CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2]);
            let trig_code1 = info[0].int() as u64;
            let trig_code2 = info[1].int() as u64;
            // table.insert(s.announce_filter,trig_code1)
            // table.insert(s.announce_filter,OPCODE_ISCODE)
            // table.insert(s.announce_filter,OPCODE_NOT)
            // table.insert(s.announce_filter,OPCODE_AND)
            announce_filter.extend_from_slice(&[trig_code1, OPCODE_ISCODE, OPCODE_NOT, OPCODE_AND]);
            // if trig_code2>0 then
            if trig_code2 > 0 {
                // table.insert(s.announce_filter,trig_code2)
                // table.insert(s.announce_filter,OPCODE_ISCODE)
                // table.insert(s.announce_filter,OPCODE_NOT)
                // table.insert(s.announce_filter,OPCODE_AND)
                announce_filter.extend_from_slice(&[trig_code2, OPCODE_ISCODE, OPCODE_NOT, OPCODE_AND]);
            }
        }
    }
    // local declared_code=Duel.AnnounceCard(tp,s.announce_filter)
    let declared_code = duel::announce_card(d, tp, &announce_filter);
    // Duel.SetTargetParam(declared_code)
    duel::set_target_param(d, declared_code as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
/// `s.declop(e,tp,eg,ep,ev,re,r,rp)`
fn declop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local declared_code=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
        let declared_code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
        // c:SetHint(CHINT_CARD,declared_code)
        card::set_hint(d, c, CHINT_CARD as u8, declared_code as u64);
        // Clear the hint at the end of the turn
        // local e0=Effect.CreateEffect(c)
        let e0 = effect::create_effect(d, c);
        // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e0:SetCode(EVENT_TURN_END)
        effect::set_code(d, e0, EVENT_TURN_END);
        // e0:SetRange(LOCATION_SZONE)
        effect::set_range(d, e0, LOCATION_SZONE);
        // e0:SetCountLimit(1)
        effect::set_count_limit(d, e0, 1, 0, 0, 0);
        // e0:SetOperation(function(e) c:SetHint(CHINT_CARD,0) e:Reset() end)
        effect::set_operation(
            d,
            e0,
            Some(lua_fn(move |d: &mut Duel, a: Args| {
                card::set_hint(d, c, CHINT_CARD as u8, 0);
                effect::reset(d, a.e(0));
            })),
        );
        // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e0)
        card::register_effect(d, c, e0, false);
        // Negate the activated effects of cards with that original name until the end of this turn, while this card is face-up on the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EVENT_CHAIN_SOLVING)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
        // e1:SetRange(LOCATION_SZONE)
        effect::set_range(d, e1, LOCATION_SZONE);
        // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp)
        //     local code1,code2=re:GetHandler():GetOriginalCodeRule()
        //     return code1==declared_code or code2==declared_code
        // end)
        effect::set_condition(
            d,
            e1,
            Some(lua_fn(move |d: &mut Duel, a: Args| {
                let re = a.e(5);
                let h = effect::get_handler(d, re).nil();
                let codes = card::get_original_code_rule(d, h);
                codes.first().copied() == Some(declared_code) || codes.get(1).copied() == Some(declared_code)
            })),
        );
        // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
        //     Duel.NegateEffect(ev)
        // end)
        effect::set_operation(
            d,
            e1,
            Some(lua_fn(|d: &mut Duel, a: Args| {
                duel::negate_effect(d, a.int(4) as u8);
            })),
        );
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
