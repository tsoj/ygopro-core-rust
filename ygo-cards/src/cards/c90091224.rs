//! Dark Rebellion Xyz Dragon of the Four Heavenly Dragons
//! 四天の龍 ダーク・リベリオン・エクシーズ・ドラゴン
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 90091224;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_PHANTOM_KNIGHTS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 DARK monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK),4,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, sc, st, Some(tp)))), Some(4), 2, None, None, None, None, false, None);
    // If this card is Special Summoned: You can target an equal number of cards on each field; destroy them
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When your opponent activates a card or effect (Quick Effect): You can detach 1 material from this card; Special Summon 1 Rank 4 DARK Xyz Monster from your Extra Deck, using this card you control (this is treated as an Xyz Summon, transfer this card's materials)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e2, Some(condition(|_d, a| a.rp == 1 - a.tp)));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desrescon(maxc)`: `function(sg,e,tp,mg) ... return ct1==ct2, ct1>maxc or ct2>maxc end`
fn desrescon(maxc: usize) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, tp, _mg, _c| {
        // local ct1=sg:FilterCount(Card.IsControler,nil,tp)
        let ct1 = group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) as usize;
        // local ct2=#sg-ct1
        let ct2 = sg.len() - ct1;
        (ct1 == ct2, ct1 > maxc || ct2 > maxc)
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, 0, 1, NO_EX) && duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetTargetGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_target_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // local ct=g:FilterCount(Card.IsControler,nil,tp)
    let ct = group::filter_count(d, &g, &|d, c| card::is_controler(d, c, tp), NO_EX) as usize;
    // local rescon=s.desrescon(math.min(ct,#g-ct))
    let n = g.len();
    let rescon = desrescon(ct.min(n - ct));
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,#g,rescon,1,tp,HINTMSG_DESTROY,rescon)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(n), Some(&rescon), 1, tp, Some(HINTMSG_DESTROY as u64), Some(&rescon), None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,#tg,tp,0)
    let m = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg), m, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).nil();
    // if #tg>0 then Duel.Destroy(tg,REASON_EFFECT) end
    if !tg.is_empty() {
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
/// `s.spfilter(c,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsRank(4) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsXyzMonster() and mc:IsCanBeXyzMaterial(c,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_rank(d, c, &[4])
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_xyz_monster(d, c)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group(d, tp, &CardSet::from_card(c), tp, None, &CardSet::new(), REASON_XYZ as u32);
        // return (#pg<=0 or (#pg==1 and pg:IsContains(c)))
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
        return (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and c:IsControler(tp) and not c:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) && card::is_controler(d, c, tp) && !card::is_immune_to_effect(d, c, e) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group(d, tp, &CardSet::from_card(c), tp, None, &CardSet::new(), REASON_XYZ as u32);
        // if not ((#pg<=0 or (#pg==1 and pg:IsContains(c)))) then return end
        if !(pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c))) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyz=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if xyz then
        if let Some(xyz) = group::get_first(d, &sel) {
            // xyz:SetMaterial(c)
            card::set_material(d, xyz, CardSet::from_card(c));
            // Duel.Overlay(xyz,c)
            duel::overlay(d, xyz, c, false);
            // if Duel.SpecialSummon(xyz,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
            if duel::special_summon(d, xyz, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // xyz:CompleteProcedure()
                card::complete_procedure(d, xyz);
            }
        }
    }
}
