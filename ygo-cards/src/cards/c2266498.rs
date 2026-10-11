//! Vendread Reunion
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 2266498;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VENDREAD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Ritual.CreateProc({handler=c,lvtype=RITPROC_EQUAL,filter=s.cfilter,extrafil=s.extrafil,matfilter=s.filter,forcedselection=s.ritcheck,customoperation=s.customoperation})
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_EQUAL,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| cfilter(d, x))),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| extrafil(d, a))),
            matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, x: CardId, e: EffectId, tp: u8| filter(d, x, e, tp)))),
            forcedselection: Some(Arc::new(|d: &mut Duel, e: EffectId, tp: u8, g: &CardSet, sc: CardId| ritcheck(d, e, tp, g, sc))),
            customoperation: Some(Arc::new(|d: &mut Duel, mg: &CardSet, a: &EffArgs, tc: CardId| customoperation(d, mg, a, tc))),
            ..Default::default()
        },
    );
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_RELEASE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_RELEASE + CATEGORY_SET);
    // e1:SetTarget(s.registerloccount(e1:GetTarget()))
    let tg = effect::get_target(d, e1).nil();
    effect::set_target(d, e1, Some(registerloccount(tg)));
    // e1:SetOperation(s.registerloccount(e1:GetOperation()))
    let op = effect::get_operation(d, e1).nil();
    effect::set_operation(d, e1, Some(registerloccount(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.registerloccount(func)
//   return function(e,tp,...)
fn registerloccount(func: LuaFn) -> LuaFn {
    lua_fn(move |d: &mut Duel, a: Args| {
        let e = a.e(0);
        let tp = a.p(1);
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // if not Duel.IsPlayerCanSpecialSummonCount(tp,2) then ft=0 end
        if !duel::is_player_can_special_summon_count(d, tp, 2) {
            ft = 0;
        }
        // if ft>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if ft > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // e:SetLabel(ft)
        effect::set_label(d, e, &[ft as i64]);
        // local res=func(e,tp,...)
        let res = func.call(d, a.0);
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return res
        res
    })
}
// function s.cfilter(c,e)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VENDREAD) and not c:IsPublic()
    card::is_set_card(d, c, &[SET_VENDREAD as u16]) && !card::is_public(d, c)
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VENDREAD) and Duel.IsPlayerCanRelease(tp,c) and c:IsLocation(LOCATION_REMOVED)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_VENDREAD as u16])
        && duel::is_player_can_release(d, tp, Some((c, None)))
        && card::is_location(d, c, LOCATION_REMOVED)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
// function s.extrafil(e,tp,eg,ep,ev,re,r,rp)
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    let (e, tp) = (a.e, a.tp);
    // return Duel.GetMatchingGroup(s.filter,tp,LOCATION_REMOVED,0,nil,e,tp)
    Some(duel::get_matching_group(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, NO_EX))
}
// function s.ritcheck(e,tp,g,sc)
fn ritcheck(d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet, _sc: CardId) -> (bool, bool) {
    // local count=#g
    let count = g.len();
    // return g:GetClassCount(Card.GetCode)==count and count <=e:GetLabel()
    // (GetClassCount counts the distinct first codes of the cards)
    let mut codes: Vec<u32> = Vec::new();
    for c in group::iter(d, g) {
        let code = card::get_code(d, c)[0];
        if !codes.contains(&code) {
            codes.push(code);
        }
    }
    (codes.len() == count && (count as i64) <= effect::get_label(d, e), false)
}
// function s.customoperation(mg,e,tp,eg,ep,ev,re,r,rp,tc)
fn customoperation(d: &mut Duel, mg: &CardSet, a: &EffArgs, tc: CardId) {
    let tp = a.tp;
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - tp, tc);
    // if #mg==0 then return end
    if mg.is_empty() {
        return;
    }
    // if Duel.SpecialSummon(mg,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)==#mg then
    if duel::special_summon(d, mg.clone(), 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) == mg.len() as i32 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // Duel.ConfirmCards(1-tp,og)
        duel::confirm_cards(d, 1 - tp, og.clone());
        // tc:SetMaterial(og)
        card::set_material(d, tc, og.clone());
        // Duel.Release(og,REASON_EFFECT+REASON_RITUAL+REASON_MATERIAL)
        duel::release(d, og, REASON_EFFECT | REASON_RITUAL | REASON_MATERIAL, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(tc,SUMMON_TYPE_RITUAL,tp,tp,false,true,POS_FACEUP)
        duel::special_summon(d, tc, SUMMON_TYPE_RITUAL, tp, tp, false, true, POS_FACEUP as u8, None);
        // tc:CompleteProcedure()
        card::complete_procedure(d, tc);
    }
}
