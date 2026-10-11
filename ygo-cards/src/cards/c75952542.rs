//! ＶＶ－百識公国 (Vaylantz World - Konig Wissen)
use super::prelude::*;

pub const ID: u32 = 75952542;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VAYLANTZ], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place in the Spell/Trap Zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_BOTH_SIDE+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_BOTH_SIDE + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.plcon)
    effect::set_condition(d, e2, Some(condition(plcon)));
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFieldSpell() and c:IsSetCard(SET_VAYLANTZ) and not c:IsCode(id) and not c:IsForbidden()
    card::is_field_spell(d, c) && card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && !card::is_code(d, c, &[ID]) && !card::is_forbidden(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // local fc=Duel.GetFieldCard(1-tp,LOCATION_FZONE,0)
        let fc = duel::get_field_card(d, 1 - tp, LOCATION_FZONE, 0);
        // if fc then
        if let Some(fc) = fc {
            // Duel.SendtoGrave(fc,REASON_RULE)
            duel::send_to_grave(d, fc, REASON_RULE, None, None);
            // Duel.BreakEffect()
            duel::break_effect(d);
        }
        // Duel.MoveToField(tc,tp,1-tp,LOCATION_FZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, tp, 1 - tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.plcon(e,tp,eg,ep,ev,re,r,rp)`
fn plcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_FZONE,LOCATION_FZONE)==2
    duel::get_field_group_count(d, a.tp, LOCATION_FZONE, LOCATION_FZONE) == 2
}
/// `s.plfilter(c,tp)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if not (c:IsFaceup() and c:IsType(TYPE_EFFECT)) then return false end
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)) {
        return false;
    }
    // local cg=c:GetColumnGroup()
    let cg = card::get_column_group(d, c, 0, 0);
    // return #cg>0 and cg:IsExists(aux.AND(Card.IsControler,Card.IsMonster),1,nil,tp)
    !cg.is_empty() && group::is_exists(d, &cg, &|d, cc| card::is_controler(d, cc, tp) && card::is_monster(d, cc), 1, NO_EX)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MMZONE) and s.plfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MMZONE) && plfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.plfilter,tp,0,LOCATION_MMZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| plfilter(d, c, tp)), tp, 0, LOCATION_MMZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.plfilter,tp,0,LOCATION_MMZONE,1,1,nil,tp):GetFirst()
    let g = duel::select_target(d, tp, Some(&|d, c| plfilter(d, c, tp)), tp, 0, LOCATION_MMZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // local dc=Duel.GetFieldCard(1-tp,LOCATION_SZONE,tc:GetSequence())
    let seq = card::get_sequence(d, tc);
    let dc = duel::get_field_card(d, 1 - tp, LOCATION_SZONE, seq);
    // if dc then
    if let Some(dc) = dc {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dc,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from(dc)), 1, 0, 0);
    }
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not (tc:IsRelateToEffect(e) and tc:IsControler(1-tp) and not tc:IsImmuneToEffect(e)) then return end
    let tc = tc.nil();
    if !(card::is_relate_to_effect(d, tc, e) && card::is_controler(d, tc, 1 - tp) && !card::is_immune_to_effect(d, tc, e)) {
        return;
    }
    // local seq=tc:GetSequence()
    let seq = card::get_sequence(d, tc);
    // local dc=Duel.GetFieldCard(1-tp,LOCATION_SZONE,seq)
    let dc = duel::get_field_card(d, 1 - tp, LOCATION_SZONE, seq);
    // if dc then Duel.Destroy(dc,REASON_RULE) end
    if let Some(dc) = dc {
        duel::destroy(d, dc, REASON_RULE, None, None);
    }
    // if Duel.CheckLocation(1-tp,LOCATION_SZONE,seq)
    //     and Duel.MoveToField(tc,tp,1-tp,LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard(),1<<seq) then
    if duel::check_location(d, 1 - tp, LOCATION_SZONE, seq)
        && duel::move_to_field(d, tc, tp, 1 - tp, LOCATION_SZONE as u16, POS_FACEUP as u8, card::is_monster_card(d, tc), Some(1u32 << seq))
    {
        // Treated as a Continuous Spell
        // local e1=Effect.CreateEffect(e:GetHandler())
        let handler = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, handler);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_TYPE)
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
        effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
