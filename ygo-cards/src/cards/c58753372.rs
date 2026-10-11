//! Super Quantal Fairy Alphan
use super::prelude::*;

pub const ID: u32 = 58753372;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SUPER_QUANT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // lv
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::has_level(d, c)
}
/// `s.filter2(c,tp)`
fn filter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return s.filter1(c) and c:IsSetCard(SET_SUPER_QUANT)
    //     and Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_MZONE,0,1,c)
    filter1(d, c)
        && card::is_set_card(d, c, &[SET_SUPER_QUANT as u16])
        && duel::is_existing_matching_card(d, Some(&filter1), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g1=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_MZONE,0,nil)
    let g1 = duel::get_matching_group(d, Some(&filter1), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter2(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter2(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,0,1,nil,tp) and g1:GetClassCount(Card.GetLevel)>1 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter2(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && group::get_class_count(d, &g1, &|d, c| card::get_level(d, c) as i64) > 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_MZONE,0,tc)
        let g = duel::get_matching_group(d, Some(&filter1), a.tp, LOCATION_MZONE, 0, tc.into());
        // local lc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // local lv=tc:GetLevel()
        let lv = card::get_level(d, tc) as i64;
        // for lc in aux.Next(g) do
        for lc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL_FINAL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // lc:RegisterEffect(e1)
            card::register_effect(d, lc, e1, false);
        }
    }
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_SUPER_QUANT as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spfilter2(c)`
fn spfilter2(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SUPER_QUANT as u16]) && card::is_monster(d, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.spfilter1,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
        // return ft>0 and g:GetClassCount(Card.GetCode)>=3
        return ft > 0 && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_DECK,0,nil)
    let mut g = duel::get_matching_group(d, Some(&spfilter2), a.tp, LOCATION_DECK, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)>=3 then
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3 {
        // local cg=Group.CreateGroup()
        let mut cg = CardSet::new();
        // for i=1,3 do
        for _ in 1..=3 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // g:Remove(Card.IsCode,nil,sg:GetFirst():GetCode())
            let code = card::get_code(d, group::get_first(d, &sg).nil())[0];
            group::remove(d, &mut g, &|d, c| card::is_code(d, c, &[code]), NO_EX);
            // cg:Merge(sg)
            group::add_card(&mut cg, &sg);
        }
        // Duel.ConfirmCards(1-tp,cg)
        duel::confirm_cards(d, 1 - a.tp, cg.clone());
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_SPSUMMON as u64);
        // local tg=cg:Select(1-tp,1,1,nil)
        let tg = group::select(d, &cg, 1 - a.tp, 1, 1, false, NO_EX).nil();
        // local tc=tg:GetFirst()
        let tc = group::get_first(d, &tg).nil();
        // if tc:IsCanBeSpecialSummoned(e,0,tp,false,false) then
        if card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            // cg:RemoveCard(tc)
            group::remove_card(&mut cg, &CardSet::from_card(tc));
        }
        // Duel.SendtoGrave(cg,REASON_EFFECT)
        duel::send_to_grave(d, cg, REASON_EFFECT, None, None);
    }
}
